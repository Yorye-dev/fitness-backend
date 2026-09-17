# fitness-backend
🦀 A lightweight and fast backend API written in Rust (Axum) for a free and open-source nutrition tracking app.


src/
├── main.rs
├── lib.rs
│
├── config/
│   └── mod.rs
│
├── app/
│   ├── mod.rs
│   └── state.rs
│
├── domain/
│   ├── mod.rs
│   ├── user.rs
│   ├── goal.rs
│   ├── meal.rs
│   └── factories/
│       ├── mod.rs
│       └── user_factory.rs
│
├── application/
│   ├── mod.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── user_service.rs
│   └── traits/
│       ├── mod.rs
│       └── user_repository.rs
│
├── infrastructure/
│   ├── mod.rs
│   ├── db/
│   │   └── postgres.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   └── postgres_user_repository.rs
│   └── auth/
│       └── jwt.rs
│
├── interfaces/
│   ├── mod.rs
│   └── http/
│       ├── mod.rs
│       ├── routes.rs
│       ├── handlers/
│       │   ├── mod.rs
│       │   └── user_handler.rs
│       └── dto/
│           ├── mod.rs
│           ├── create_user_request.rs
│           └── user_response.rs
│
└── errors/
    ├── mod.rs
    └── app_error.rsº
