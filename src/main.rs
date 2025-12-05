use librust_winrm::{WinRMClient, upload_file, download_file};
use clap::{Parser, Subcommand};
use anyhow::Result;
use std::sync::atomic::{AtomicU8, Ordering};


// Exit codes
const EXIT_SUCCESS: i32 = 0;
const EXIT_AUTH_ERROR: i32 = 1;
const EXIT_CONNECTION_ERROR: i32 = 2;
const EXIT_COMMAND_ERROR: i32 = 3;
const EXIT_FILE_ERROR: i32 = 4;

// Global verbosity level: 0=quiet (default), 1=normal, 2=verbose
pub static VERBOSITY: AtomicU8 = AtomicU8::new(0);

pub fn log_info(msg: &str) {
    if VERBOSITY.load(Ordering::Relaxed) >= 1 {
        println!("{}", msg);
    }
}

pub fn log_verbose(msg: &str) {
    if VERBOSITY.load(Ordering::Relaxed) >= 2 {
        println!("[VERBOSE] {}", msg);
    }
}

#[derive(Parser)]
#[command(
    name = "winrm-client",
    author = "WinRM Client Contributors",
    version,
    about = "A WinRM client for remote Windows management",
    long_about = "Execute commands and transfer files on remote Windows systems using WinRM protocol.

EXAMPLES:
    # Execute a command with NTLM
    winrm-client -e 10.0.3.203 -u admin -p pass --encrypt --insecure command \"whoami\"
    
    # Upload/download files
    winrm-client -e server -u admin -p pass upload local.txt C:\\remote.txt
    winrm-client -e server -u admin -p pass download C:\\file.txt ./local.txt
    
    # Using environment variables (no parameters needed)
    export WINRM_ENDPOINT=10.0.3.203
    export WINRM_USER=admin
    export WINRM_PASSWORD=secret
    export WINRM_ENCRYPT=true
    winrm-client command \"hostname\"

ENVIRONMENT VARIABLES:
    WINRM_ENDPOINT    - Server endpoint (IP or hostname)
    WINRM_USER        - Username
    WINRM_PASSWORD    - Password
    WINRM_AUTH        - Auth method: ntlm, basic, kerberos (default: ntlm)
    WINRM_ENCRYPT     - Use HTTPS (true or false)
    WINRM_INSECURE    - Skip SSL cert validation (true or false)
    WINRM_VERBOSE     - Verbose output (true or false)

    Note: Boolean flags accept only 'true' or 'false' (lowercase)

PARAMETERS:
    -e, --endpoint    WinRM endpoint
    -u, --user        Username
    -p, --password    Password
    -a, --auth        Auth method (ntlm/basic/kerberos)
    -k, --insecure    Skip SSL validation
    -v, --verbose     Detailed logs

EXIT CODES:
    0 - Success
    1 - Authentication error
    4 - File transfer error"
)]
struct Cli {
    /// WinRM endpoint (e.g., 10.0.3.203 or https://server:5986/wsman)
    #[arg(short = 'e', long, env = "WINRM_ENDPOINT")]
    endpoint: String,

    /// Username for authentication
    #[arg(short = 'u', long, env = "WINRM_USER")]
    user: String,

    /// Password for authentication
    #[arg(short = 'p', long, env = "WINRM_PASSWORD")]
    password: String,

    /// Authentication method: ntlm, basic, kerberos
    #[arg(short = 'a', long, default_value = "ntlm", env = "WINRM_AUTH")]
    #[arg(value_parser = ["ntlm", "basic", "kerberos"])]
    auth: String,

    /// Use HTTPS (port 5986) with encryption
    #[arg(long, conflicts_with = "no_encrypt", env = "WINRM_ENCRYPT")]
    encrypt: bool,

    /// Use HTTP (port 5985) without TLS
    #[arg(long, env = "WINRM_NO_ENCRYPT")]
    no_encrypt: bool,

    /// Skip SSL certificate validation
    #[arg(short = 'k', long, env = "WINRM_INSECURE")]
    insecure: bool,

    /// Path to CA certificate
    #[arg(long)]
    cacert: Option<String>,

    /// Verbose output (detailed logs)
    #[arg(short = 'v', long, env = "WINRM_VERBOSE")]
    verbose: bool,

    /// Quiet mode (suppress all output except errors and command output)
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a command on the remote system
    Command {
        /// Command to execute
        cmd: String,
    },
    /// Upload a file to the remote system
    Upload {
        /// Local file path
        local: String,
        /// Remote file path
        remote: String,
    },
    /// Download a file from the remote system
    Download {
        /// Remote file path
        remote: String,
        /// Local file path
        local: String,
    },
}

fn adjust_endpoint(endpoint: &str, encrypt: bool, no_encrypt: bool) -> String {
    let mut ep = endpoint.to_string();

    if encrypt {
        if ep.starts_with("http://") {
            ep = ep.replace("http://", "https://");
        } else if !ep.starts_with("https://") {
            ep = format!("https://{}", ep);
        }
        if ep.contains(":5985") {
            ep = ep.replace(":5985", ":5986");
        } else if !ep.contains(":5986") && !ep.contains(":443") && ep.matches(':').count() < 2 {
            ep = format!("{}:5986", ep);
        }
    } else if no_encrypt {
        if ep.starts_with("https://") {
            ep = ep.replace("https://", "http://");
        } else if !ep.starts_with("http://") {
            ep = format!("http://{}", ep);
        }
        if ep.contains(":5986") {
            ep = ep.replace(":5986", ":5985");
        } else if !ep.contains(":5985") && !ep.contains(":80") && ep.matches(':').count() < 2 {
            ep = format!("{}:5985", ep);
        }
    } else {
        if !ep.starts_with("http://") && !ep.starts_with("https://") {
            ep = format!("http://{}", ep);
        }
        if !ep.contains(":5985") && !ep.contains(":5986") && !ep.contains(":80") && !ep.contains(":443") && ep.matches(':').count() < 2 {
            if ep.starts_with("https://") {
                let host_part = &ep[8..];
                ep = format!("https://{}:5986", host_part.trim_end_matches("/wsman"));
            } else {
                let host_part = &ep[7..];
                ep = format!("http://{}:5985", host_part.trim_end_matches("/wsman"));
            }
        }
    }

    if !ep.ends_with("/wsman") {
        ep = format!("{}/wsman", ep);
    }

    ep
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // Set verbosity: default=0 (quiet), -v=2 (verbose)
    if cli.verbose {
        VERBOSITY.store(2, Ordering::Relaxed);
    }
    // Note: quiet flag artık gereksiz, varsayılan zaten quiet
    
    let endpoint = adjust_endpoint(&cli.endpoint, cli.encrypt, cli.no_encrypt);

    log_info(&format!("Connecting to {} ({})...", endpoint, cli.auth));

    let mut client = WinRMClient::new(
        &endpoint,
        &cli.user,
        &cli.password,
        &cli.auth,
        cli.insecure,
        cli.cacert,
    )?;

    let shell_id = client.open_shell()?;

    let result = match &cli.command {
        Commands::Command { cmd } => {
            log_info(&format!("Executing command: {}", cmd));
            let command_id = client.run_command(&shell_id, cmd)?;
            let (stdout, stderr, exit_code) = client. get_command_output(&shell_id, &command_id)?;
            print!("{}", stdout);
            eprint!("{}", stderr);
            if exit_code != 0 {
                Err(anyhow::anyhow!("Command failed with exit code {}", exit_code))
            } else {
                Ok(())
            }
        }
        Commands::Upload { local, remote } => {
            upload_file(&mut client, &shell_id, local, remote)
        }
        Commands::Download { remote, local } => {
            download_file(&mut client, &shell_id, remote, local)
        }
    };

    client.close_shell(&shell_id)?;

    result
}

fn main() {
    let exit_code = match run() {
        Ok(_) => EXIT_SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            
            // Determine exit code based on error type
            if let Some(winrm_err) = e.downcast_ref::<librust_winrm::WinRMError>() {
                match winrm_err {
                    librust_winrm::WinRMError::AuthenticationFailed { .. } => EXIT_AUTH_ERROR,
                    librust_winrm::WinRMError::ConnectionError { .. } => EXIT_CONNECTION_ERROR,
                    librust_winrm::WinRMError::InvalidResponse { .. } => EXIT_CONNECTION_ERROR,
                    librust_winrm::WinRMError::FileTransferError { .. } => EXIT_FILE_ERROR,
                    _ => EXIT_COMMAND_ERROR,
                }
            } else {
                EXIT_COMMAND_ERROR
            }
        }
    };
    
    std::process::exit(exit_code);
}
