# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [v] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [v] Commit: `Create Subscriber model struct.`
    -   [v] Commit: `Create Notification model struct.`
    -   [v] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [v] Commit: `Implement add function in Subscriber repository.`
    -   [v] Commit: `Implement list_all function in Subscriber repository.`
    -   [v] Commit: `Implement delete function in Subscriber repository.`
    -   [v] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [v] Commit: `Create Notification service struct skeleton.`
    -   [v] Commit: `Implement subscribe function in Notification service.`
    -   [v] Commit: `Implement subscribe function in Notification controller.`
    -   [v] Commit: `Implement unsubscribe function in Notification service.`
    -   [v] Commit: `Implement unsubscribe function in Notification controller.`
    -   [v] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. Berdasarkan pemahaman saya tentang pola Observer, untuk kasus BambangShop ini, kita tidak memerlukan interface atau trait di Rust karena hanya ada satu tipe observer yaitu Subscriber. Interface atau trait biasanya diperlukan ketika terdapat beberapa jenis observer yang berbeda-beda yang perlu mengimplementasikan metode yang sama. Dalam kasus ini, struct Model tunggal sudah cukup karena semua subscriber memiliki perilaku yang sama dan tidak ada variasi implementasi yang diperlukan.

2. Dalam kasus ini, penggunaan DashMap lebih tepat dibandingkan Vec karena id dalam Program dan url dalam Subscriber dimaksudkan untuk bersifat unik. DashMap memungkinkan kita untuk menyimpan data dalam struktur key-value, sehingga kita dapat dengan mudah mencari, menambah, atau menghapus subscriber berdasarkan id atau url mereka. Jika kita menggunakan Vec, kita harus melakukan iterasi manual untuk mencari elemen yang spesifik, yang bisa menjadi tidak efisien terutama ketika jumlah subscriber menjadi besar. Selain itu, DashMap juga mempermudah untuk memastikan keunikan data karena kita dapat memeriksa keberadaan key sebelum menyimpan data baru.

3. Dalam pemrograman Rust, kita memang diharuskan oleh compiler untuk membuat program yang thread-safe. Untuk variabel static SUBSCRIBERS, penggunaan DashMap sebagai library eksternal untuk HashMap yang thread-safe memang diperlukan. Kita masih membutuhkan DashMap meskipun kita mengimplementasikan pola Singleton, karena Singleton hanya memastikan bahwa hanya ada satu instance dari kelas tertentu, tetapi tidak otomatis membuat akses ke data di dalamnya menjadi thread-safe. DashMap memberikan mekanisme concurrent yang aman untuk akses baca dan tulis secara bersamaan, yang penting dalam konteks multi-thread. Jadi, untuk kasus ini, implementasi Singleton tidak menggantikan kebutuhan akan DashMap, melainkan keduanya dapat digunakan bersama-sama untuk memastikan single instance yang thread-safe.

#### Reflection Publisher-2

1. Berdasarkan pemahaman saya tentang prinsip desain software, pemisahan Model menjadi "Service" dan "Repository" sangat penting untuk menerapkan prinsip Single Responsibility Principle (SRP) dari SOLID. Dengan separasi ini, Repository bertanggung jawab khusus untuk operasi penyimpanan data dan interaksi dengan database, sementara Service berfokus pada logika bisnis dan aturan aplikasi. Pemisahan ini membuat kode lebih terstruktur, mudah diuji (testable), dan lebih mudah dimaintain. Ketika kita perlu mengubah cara penyimpanan data, kita hanya perlu memodifikasi Repository tanpa menyentuh logika bisnis di Service, dan sebaliknya.

2. Jika kita hanya menggunakan Model tanpa pemisahan Service dan Repository, kompleksitas kode akan meningkat secara signifikan. Bayangkan jika ketiga model kita (Program, Subscriber, Notification) harus menangani penyimpanan data, logika bisnis, dan validasi secara bersamaan. Setiap model akan menjadi sangat besar dan sulit dipahami. Selain itu, akan terjadi banyak duplikasi kode karena logika yang serupa mungkin perlu diimplementasikan di beberapa model. Interaksi antar model juga akan lebih rumit karena setiap model perlu mengetahui detail implementasi model lainnya. Misalnya, jika Program perlu mengirim notifikasi ke Subscriber, tanpa pemisahan Service, Program harus mengetahui cara mengakses dan memperbarui data Notification secara langsung, yang meningkatkan coupling dan mengurangi maintainability kode.

3. Dengan menggunakan Postman, kita dapat mengamati dan memverifikasi apakah perilaku aplikasi sudah sesuai dengan ekspektasi kita. Tool ini memungkinkan kita untuk mengirimkan berbagai jenis HTTP method, menambahkan credential autentikasi, menyertakan form data, sertaa elemen-elemen request lainnya. Selain itu, Postman akan menampilkan response dari server, sehingga memudahkan kita untuk mengevaluasi apakah endpoint berfungsi dengan benar sesuai harapan. Kemampuan ini sangat berharga dalam proses pengembangan karena kita dapat dengan cepat memastikan endpoint kita bekerja sebagaimana mestinya.

#### Reflection Publisher-3
