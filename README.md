# BambangShop Receiver App
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
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [v] Commit: `Create Notification model struct.`
    -   [v] Commit: `Create SubscriberRequest model struct.`
    -   [v] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [v] Commit: `Implement add function in Notification repository.`
    -   [v] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [v] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
1. Alasan kita menggunakan Read Write Lock dibanding dengan MUTEX pada kasus ini adalah Rwlock dibuat untuk mengakomodasi banyak baca sekaligus sedangkan mutex hanya memperbolehkan 1 thread yang menggunakan variabel saat suatu waktu sehingga pada kasus ini lebih cocok karena vector Notifications akan dibaca oleh banyak thread sekaligus tanpa penulisan. Jika digunakan mutex maka hal tersebut tidak memungkikan untuk dilakukan.

2. lazy_static berfungsi untuk membuat suatu variabel menjadi singleton dimana artinya variabel tersebut hanya akan ada 1 dalam program tersebut. Selain itu variabel static di rust dibuat immutable oleh rust tidak seperti di Java yang bisa berubah-ubah datanya untuk menjamin thread safety saat multi threading. 

#### Reflection Subscriber-2
1. Menurut saya, lib.rs berisikan informasi-informasi yang diperlukan oleh bagian-bagian pada aplikasi lainnya. Seperti Error Response, root URL dan singleton dari App Configuration.

2. Ya, dikarenakan desain penulisan kode yang telah dilakukan, penambahan jenis observer baru akan lebih mudah ditambahkan karena program bersifat open close untuk perubahan tersebut. Untuk kasus instansiasi lebih dari 1 Main App, tetap bisa dilakukan karena tinggal masalah mendaftarkan subscriber terhadap aplikasi yang berbeda dengan cara meng-hit API yang sesuai.

3. Ya, menurut saya sangat berguna. Dengan adanya testing atau collection Postman, kita jadi bisa mengetahui kebenaran program kita. Collection Postman juga membantu dalam memverifikasi apakah program mengirimkan response dan berjalan sesuai dengan harapan kita atau tidak karena menggunakan data asli pada aplikasinya.