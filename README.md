# 🚀 Mastering High-Level Networking: A Journey into Rust gRPC

***by Christna Yosua Rotinsulu - 2406495691***

Repositori ini berisi eksplorasi dan implementasi saya mengenai sistem *Remote Procedure Call* performa tinggi menggunakan **gRPC** dengan bahasa pemrograman **Rust** (melalui *framework* **Tonic**). Melalui proyek ini, saya mensimulasikan tiga layanan utama: `PaymentService`, `TransactionService`, dan `ChatService`.

Di bawah ini adalah refleksi saya mengenai berbagai konsep yang telah saya pelajari selama pengerjaan proyek ini.

---

## 1. 🔄 Unary vs. Streaming: Choosing the Right gRPC Paradigm

Berdasarkan implementasi yang saya lakukan, terdapat perbedaan mendasar dari ketiga mekanisme gRPC. Pada metode **Unary**, klien mengirim satu permintaan dan menerima satu balasan dari server. Saya menggunakannya untuk `PaymentService` karena operasi pembayaran hanya butuh konfirmasi sukses atau gagal secara instan.

Sebaliknya, pada **Server Streaming**, klien meminta satu kali, namun server merespons dengan aliran data (*stream*) berkelanjutan. Ini sangat ideal untuk mengirim data berukuran besar secara bertahap, seperti saat saya merancang `TransactionService` untuk mengambil 30 riwayat transaksi secara beruntun. Sedangkan pada **Bi-directional Streaming**, klien dan server berbagi saluran terbuka dan dapat saling mengirim pesan secara bersamaan tanpa saling menunggu. Pola ini sempurna untuk sistem interaktif waktu-nyata seperti `ChatService` yang saya buat.

## 2. 🛡️ Securing the Gates: Authentication and Encryption in Rust gRPC

Saat membangun layanan gRPC dengan Rust ini, saya menyadari ada beberapa pertimbangan keamanan yang harus saya perhatikan agar sistem siap untuk masuk ke tahap produksi. Pertama, saya perlu memastikan koneksi jaringan dibungkus oleh enkripsi **TLS** (*Transport Layer Security*) sehingga paket data transaksi tidak bisa disadap di tengah jalan. Kedua, saat ini metode layanan saya dapat diakses oleh siapa saja secara bebas. Di sistem nyata, saya harus menambahkan otentikasi dan otorisasi, misalnya dengan mencegat token **JWT** melalui *metadata header* untuk memastikan bahwa klien tersebut memang berhak menginisiasi komunikasi.

## 3. 🔀 Taming the Chaos: Challenges of Bi-directional Streaming

Dalam proses penulisan kode untuk `ChatService`, tantangan terbesar saya adalah mengelola aliran komunikasi dua arah yang berjalan bersamaan. **Rust** menuntut saya memikirkan kepemilikan variabel (*ownership*) dengan sangat ketat agar tidak terjadi kebocoran memori. Saya harus mengisolasi proses penerimaan pesan ke dalam tugas asinkron terpisah menggunakan `tokio::spawn`, sambil secara bersamaan mengatur pengiriman balasan menggunakan saluran `mpsc::channel`:

```rust
let (tx, rx) = mpsc::channel(10);

tokio::spawn(async move {
    while let Some(message) = stream.message().await.unwrap_or_else(|_| None) {
        println!("Received message: {:?}", message);
        // Logika penerimaan dan pengiriman pesan...
        tx.send(Ok(reply)).await.unwrap_or_else(|_| {});
    }
});
```

## 4. 🌊 Riding the Wave: The Pros and Cons of ReceiverStream

Saya memanfaatkan `tokio_stream::wrappers::ReceiverStream` untuk membungkus jalur antrean komunikasi lokal menjadi aliran data yang dapat diproses langsung oleh **Tonic gRPC**. Kelebihannya, antarmuka ini sangat memudahkan saya menjembatani saluran data asinkron di dalam memori menjadi format jaringan standar. Namun kekurangannya, terdapat sedikit tambahan beban kerja komputasi (*overhead*) karena sistem harus membungkus struktur data berulang kali. Jika saya keliru menetapkan batas ukuran antrean `mpsc::channel(4)`, aliran data berpotensi mengalami penyumbatan.

## 5. 🧱 Building for the Future: Structuring Rust gRPC for Modularity

Saat ini, semua kerangka implementasi layanan saya, seperti `MyPaymentService` dan `MyTransactionService`, tergabung dalam satu file `grpc_server.rs` yang sama. Jika skala fitur proyek ini membesar, hal ini akan sangat menyulitkan perbaikan dan pemeliharaan. Untuk menjaga arsitektur perangkat lunak yang baik, saya seharusnya memecah fungsi-fungsi logika bisnis ke dalam file modul yang terpisah. Dengan begitu, lapisan kode gRPC benar-benar murni hanya bertugas mengatur penerimaan dan pengiriman jaringan.

## 6. 💳 Beyond the Basics: Leveling Up MyPaymentService

Saat ini, fungsi pemrosesan pembayaran yang saya buat masih sekadar mock-up sederhana, di mana ia selalu instan mengembalikan status `true`:

```rust
async fn process_payment(
    &self,
    request: Request<PaymentRequest>,
) -> Result<Response<PaymentResponse>, Status> {
    Ok(Response::new(PaymentResponse { success: true }))
}
```

Untuk skenario level produksi, saya wajib memvalidasi apakah jumlah saldonya adalah angka positif yang valid, memanggil logika basis data untuk memperbarui saldo secara fisik, dan mengirimkan datanya ke API pembayaran eksternal pihak ketiga. Saya juga harus mengubah implementasi fungsi agar bisa memunculkan status galat gRPC (seperti `Status::internal` atau `Status::invalid_argument`) jika pemrosesan transaksi gagal.

## 7. 🌐 The Bigger Picture: gRPC's Impact on Distributed Systems

Eksplorasi gRPC menggunakan ekosistem Rust ini memperlihatkan kepada saya bagaimana penerapan kontrak komunikasi yang mengikat dapat menyederhanakan arsitektur sistem terdistribusi. Dengan menggunakan definisi `services.proto`, saya menjamin bahwa antarmuka peladen (*backend*) saya secara spesifik sejalan dengan struktur klien. Hal ini menjamin kelancaran interaksi antar layanan (*microservices*) secara presisi, bahkan ketika klien dan peladen menggunakan bahasa pemrograman yang berbeda secara ekstrim.

## 8. ⚡ The Protocol Showdown: HTTP/2 (gRPC) vs. HTTP/1.1 (REST)

Pengalaman menggunakan gRPC membuat saya melihat kelemahan utama dari arsitektur REST tradisional. Karena gRPC berjalan secara eksklusif pada protokol **HTTP/2**, sistem yang saya bangun mendapat keuntungan bawaan berupa pengiriman ribuan pesan berbarengan di atas koneksi tunggal tanpa antrean macet (*multiplexing*), serta pemampatan kerangka data biner yang super efisien. Sayangnya, efisiensi biner ini membuat saya tidak bisa melacak atau membaca langsung aliran datanya saat penelusuran galat (*debugging*), tidak seperti format teks **JSON** yang mudah dibaca secara langsung.

## 9. ⏱️ Real-time Dynamics: REST Polling vs. gRPC Bi-directional Streams

Jika saya memaksa membuat fitur obrolan pelanggan menggunakan model interaksi REST API (**HTTP/1.1**), klien akan terpaksa membanjiri peladen saya dengan metode *polling* setiap dua atau tiga detik hanya untuk mengecek apakah ada balasan baru. Dengan mengadopsi mekanisme **Bi-directional streaming** pada gRPC, sistem mampu mempertahankan efisiensi secara drastis dengan menahan saluran komunikasi tetap konstan. Hal ini secara instan mengurangi beban kelatenan jaringan dan konsumsi perangkat keras.

## 10. 📜 The Contract vs. The Free-Spirit: Protobuf vs. JSON

Berkas struktur definisi **Protocol Buffers** (`.proto`) memaksa saya untuk secara kaku mendefinisikan seluruh properti variabel di awal. Pendekatan berbasis skema ini sangat bertentangan dengan kebebasan antarmuka REST (melalui **JSON**) di mana saya bebas menginjeksikan struktur variabel apa saja sewaktu-waktu. Walaupun **JSON** sangat ramah pengguna dalam tahap awal rintisan proyek, disiplin **Protobuf** pada akhirnya membantu sistem saya kebal terhadap malfungsi yang disebabkan ketidaksesuaian tipe data.

---

## 📚 References

*   **gRPC Authors.** (2026). *Core concepts, architecture and lifecycle*. gRPC Documentation. Retrieved May 5, 2026, from [https://grpc.io/docs/what-is-grpc/core-concepts/](https://grpc.io/docs/what-is-grpc/core-concepts/)
*   **Tonic Developers.** (2026). *Tonic: A native gRPC client & server implementation with async/await support*. Docs.rs. Retrieved May 5, 2026, from [https://docs.rs/tonic/latest/tonic/](https://docs.rs/tonic/latest/tonic/)