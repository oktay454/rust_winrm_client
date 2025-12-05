# Rust WinRM Client

Windows Remote Management (WinRM) client written in Rust.

## Özellikler

- ✅ **NTLM Authentication** - Tam destek
- ✅ **Remote Command Execution** - PowerShell ve CMD
- ✅ **File Transfer** - Upload & Download
- ✅ **Improved Error Handling** - Detaylı hata mesajları
- ✅ **CLI İyileştirmeleri** - Exit codes, env vars, verbose/quiet modlar
- ⚠️ **Kerberos** - Stub (bilgilendirici mesaj)

## Kurulum

```bash
cargo build --release
```

Binary: `target/release/rust_winrm_client`

## Kullanım

### Basit Komut

```bash
# Kısa parametreler ile
winrm-client -e 10.0.3.203 -u administrator -p "password" --encrypt --insecure command "whoami"

# Uzun parametreler ile
winrm-client --endpoint server --user admin --password pass command "ipconfig"
```

### Dosya Transfer

```bash
# Upload
winrm-client -e server -u admin -p pass upload local.txt C:\\remote.txt

# Download
winrm-client -e server -u admin -p pass download C:\\file.txt ./local.txt
```

### Ortam Değişkenleri

```bash
# Ortam değişkenlerini ayarla
export WINRM_ENDPOINT=10.0.3.203
export WINRM_USER=administrator
export WINRM_PASSWORD="secretpass"
export WINRM_AUTH=ntlm          # ntlm, basic, kerberos
export WINRM_ENCRYPT=true       # HTTPS kullan
export WINRM_INSECURE=true      # SSL sertifika doğrulamasını atla

# Parametresiz çalıştır
winrm-client command "whoami"
```

**Boolean Değerler:**  
- ✅ `true` veya `false` (büyük/küçük harf duyarlı)
- ❌ `1`, `0`, `yes`, `no` çalışmaz

### Verbose/Quiet Modlar

```bash
# Sessiz mod (varsayılan) - sadece komut çıktısı
winrm-client -e server command "test"

# Verbose mod - detaylı loglar
winrm-client -v -e server command "test"
```

## Parametreler

| Kısa | Uzun | Env Var | Açıklama |
|------|------|---------|----------|
| `-e` | `--endpoint` | `WINRM_ENDPOINT` | WinRM endpoint (IP veya hostname) |
| `-u` | `--user` | `WINRM_USER` | Kullanıcı adı |
| `-p` | `--password` | `WINRM_PASSWORD` | Parola |
| `-a` | `--auth` | `WINRM_AUTH` | Auth metodu (ntlm/basic/kerberos) |
| | `--encrypt` | `WINRM_ENCRYPT` | HTTPS kullan (port 5986) |
| | `--no-encrypt` | `WINRM_NO_ENCRYPT` | HTTP kullan (port 5985) |
| `-k` | `--insecure` | `WINRM_INSECURE` | SSL sertifika doğrulamasını atla |
| `-v` | `--verbose` | `WINRM_VERBOSE` | Verbose output |

## Exit Kodları

| Kod |  Anlamı |
|-----|---------|
| 0 | Başarılı |
| 1 | Authentication hatası |
| 2 | Connection hatası |
| 3 | Command execution hatası |
| 4 | File transfer hatası |

```bash
# Exit code kontrolü
winrm-client -e server command "test"
echo $?  # Exit code'u göster
```

## Örnekler

### HTTPS ile NTLM
```bash
winrm-client -e 10.0.3.203 -u administrator -p "Admin789" \
  --encrypt --insecure command "hostname"
```

### Env Vars ile
```bash
export WINRM_ENDPOINT=10.0.3.203
export WINRM_USER=admin
export WINRM_PASSWORD=pass
export WINRM_ENCRYPT=true
export WINRM_INSECURE=true

winrm-client command "whoami"
```

### PowerShell Komutları
```bash
winrm-client -e server command "Get-Process | Select-Object -First 5"
```

## Bilinen Sınırlamalar

- **Kerberos**: Tam implementasyon yok (stub mesaj gösterir)
- **HTTP Message Encryption**: HTTP (5985) için app-level encryption yok - HTTPS kullanın
- **CredSSP**: Desteklenmiyor

## Katkıda Bulunma

Pull request'ler memnuniyetle karşılanır!

## Lisans

GPL-3.0-or-later
