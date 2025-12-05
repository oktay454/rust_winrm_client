# Rust WinRM Client

Rust ile yazılmış Windows Uzaktan Yönetim (WinRM) istemcisi.

## Özellikler

- ✅ **NTLM Kimlik Doğrulama** - Tam destek
- ✅ **Uzantan Komut Çalıştırma** - PowerShell ve CMD
- ✅ **Dosya Aktarımı** - İndirme & Gönderme
- ✅ **Geliştirilmiş Hata İşleme** - Detaylı hata iletileri
- ✅ **CLI İyileştirmeleri** - Çıkış kodları, ortam değişkenleri, detaylı/sessiz çıktılar
- ⚠️ **Kerberos** - Stub (bilgilendirici mesaj)

## Kurulum

```bash
cargo build --release
```

Çalıştırılabilir ikilik dosya: `target/release/rust_winrm_client`

## Kullanım

### Basit Komut

```bash
# Kısa parametreler ile
winrm-client -e 10.0.3.203 -u administrator -p "password" --encrypt --insecure command "whoami"

# Uzun parametreler ile
winrm-client --endpoint server --user admin --password pass command "ipconfig"
```

### Dosya Aktarımı

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

### Detaylı/Sessiz Kipler

```bash
# Sessiz kip (varsayılan) - sadece komut çıktısı
winrm-client -e server command "test"

# Verbose kip - detaylı çıktılar
winrm-client -v -e server command "test"
```

## Parametreler

| Kısa | Uzun | Ortam Değişkeni | Açıklama |
|------|------|---------|----------|
| `-e` | `--endpoint` | `WINRM_ENDPOINT` | WinRM bağlantı noktası (IP veya hostname) |
| `-u` | `--user` | `WINRM_USER` | Kullanıcı adı |
| `-p` | `--password` | `WINRM_PASSWORD` | Parola |
| `-a` | `--auth` | `WINRM_AUTH` | Kimlik doğrulama yöntemi (ntlm/basic/kerberos) |
| | `--encrypt` | `WINRM_ENCRYPT` | HTTPS kullan (port 5986) |
| | `--no-encrypt` | `WINRM_NO_ENCRYPT` | HTTP kullan (port 5985) |
| `-k` | `--insecure` | `WINRM_INSECURE` | SSL sertifika doğrulamasını atla |
| `-v` | `--verbose` | `WINRM_VERBOSE` | Detaylı çıktı |

## Çıkış Kodları

| Kod |  Anlamı |
|-----|---------|
| 0 | Başarılı |
| 1 | Kimlik doğrulama hatası |
| 2 | Bağlantı hatası |
| 3 | Komut çalıştırma hatası |
| 4 | Dosya aktarım hatası |

```bash
# Çıkış kodu denetimi
winrm-client -e server command "test"
echo $?  # Exit code'u göster
```

## Örnekler

### HTTPS ile NTLM
```bash
winrm-client -e 10.0.3.203 -u administrator -p "Admin789" \
  --encrypt --insecure command "hostname"
```

### Ortam Değişkenleri ile
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

- **Kerberos**: Tam uygulama yok (stub mesaj gösterir)
- **HTTP Message Encryption**: HTTP (5985) için uygulama seviyesi şifreleme yok - HTTPS kullanın
- **CredSSP**: Desteklenmiyor

## Katkıda Bulunma

Katkılar memnuniyetle karşılanır!

## Lisans

GPL-3.0-or-later
