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
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
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
1. I've realized that while the classic Head First Design Patterns book uses an interface for the Subscriber, a single Model struct is honestly enough for our specific BambangShop case. Since all our subscribers are basically just external URLs that receive the exact same HTTP POST payload, we don't really have different "types" of subscribers that require polymorphic behavior right now. If we later decide to add different ways of notifying people—like sending an email or an SMS instead of just hitting a webhook—then introducing a Rust trait would definitely make sense. But for now, keeping it as a simple struct makes the codebase cleaner and avoids unnecessary over-engineering.
2. I think using a DashMap (or any map/dictionary) is necessary compared to just using a standard Vec. Though list technically work, it would be annoying for performance because every time someone unsubscribes, the program would have to loop through the entire list to find the matching URL, which is an O(n) operation. By using a dictionary where the URL acts as the unique key, we can instantly look up, add, or delete a subscriber  in O(1) time. This makes the application way more efficient and scalable, especially if the shop ends up getting thousands of subscribers.
3. When it comes to Rust's compiler constraints for thread safety, I think there's a bit of a mix-up between what a Singleton does and what DashMap does, so we actually need both concepts working together. Implementing the Singleton pattern just ensures we have one globally shared instance of our database across the whole app, which we are actually already doing using the lazy_static! macro. However, because web frameworks handle requests concurrently on multiple threads, multiple users might try to subscribe or unsubscribe at the exact same millisecond. If we just used a regular HashMap inside our Singleton, Rust's compiler would immediately stop us to prevent data races. We still need the DashMap external library to safely manage those concurrent.

#### Reflection Publisher-2
1. If we just stick everything into the Model like in traditional MVC, the file gets way too bloated. By separating Service (for the actual business logic) and Repository (strictly for database stuff), the code becomes much cleaner and follows the Single Responsibility Principle. If I need to fix how a notification is saved to the database, I know exactly to look in the repository file without accidentally messing up the core logic in the service layer.
2. If we only used the Model, the interactions would turn chaotic. The Program model would have to be tightly coupled to Subscriber and Notification just to figure out how to send updates. If someone changed a small detail in how Subscriber works, it could easily break the Program model. Separating them means they don't have to know the inner workings of each other, they just communicate through the service layer.
3. I’ve definitely found Postman super helpful because it lets me test my backend routes without having to build a frontend first. Just being able to quickly send a POST request with a JSON body and instantly see if it works saves so much time. For the group project, I think the feature to save requests into a shared Collection is going to be a lifesaver, because everyone on the team will know exactly what the payload should look like without having to guess or read through the Rust code.

#### Reflection Publisher-3
