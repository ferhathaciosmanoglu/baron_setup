# BARON Tauri Desktop v1.2.0 — Otomatik Güncelleme

Bu sürümde Tauri v2'nin imzalı updater sistemi GitHub Releases ile birlikte etkinleştirildi.

## Eklenenler

- Uygulama açılışında otomatik güncelleme kontrolü
- Yeni sürüm bulunduğunda **Şimdi Güncelle / Daha Sonra** ekranı
- İndirme yüzdesi ve ilerleme çubuğu
- Güncelleme notlarının gösterilmesi
- Sistem tepsinden manuel **Güncellemeleri Kontrol Et** seçeneği
- İmzalı NSIS updater paketi
- GitHub Actions ile Setup, `.sig` ve `latest.json` üretimi
- Güncelleme tamamlanınca Windows installer tarafından otomatik kapanış ve kurulum

## İlk GitHub kurulumu

`GITHUB_SECRETS_KURULUM.txt` dosyasındaki adımları uygula. Özel anahtar:

`updater-keys/baron-updater.key`

Bu dosyayı kesinlikle GitHub reposuna yükleme. `.gitignore` içine eklenmiştir.

## Bilgisayarda imzalı Setup oluşturma

`BARON_Tauri_Setup_Olustur.bat` dosyasına çift tıkla.

Çıktı:

`src-tauri/target/release/bundle/nsis/`

## GitHub üzerinden yeni sürüm yayınlama

Üç dosyadaki sürüm aynı olmalıdır:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

Ardından örneğin:

```bash
git add .
git commit -m "BARON v1.2.0"
git push
git tag v1.2.0
git push origin v1.2.0
```

GitHub Actions otomatik Release oluşturur. Uygulamalar şu endpoint'i kontrol eder:

`https://github.com/ferhathaciosmanoglu/baron_setup/releases/latest/download/latest.json`

## Güvenlik

Tauri updater imzasız güncellemeyi kabul etmez. Açık anahtar uygulamaya gömülüdür; özel anahtar yalnızca sende ve GitHub Actions Secret alanında bulunmalıdır.
