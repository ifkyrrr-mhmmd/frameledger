#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct MovieReview {
    pub id: u32,
    pub movie_id: String, // ID dari external database (TMDB/IMDb)
    pub title: String,
    pub rating: u32,      // Skala 1-100 untuk presisi
    pub review: String,
    pub timestamp: u64,   // Waktu pencatatan (Blockchain time)
}

// Key untuk storage
const REVIEWS: Symbol = symbol_short!("REVIEWS");
const COUNTER: Symbol = symbol_short!("REV_COUNT");

#[contract]
pub struct FrameLedgerContract;

#[contractimpl]
impl FrameLedgerContract {
    
    // 1. Fungsi untuk mencatat ulasan film baru
    pub fn add_review(env: Env, movie_id: String, title: String, rating: u32, review: String) -> u32 {
        let mut reviews: Vec<MovieReview> = env.storage().instance().get(&REVIEWS).unwrap_or(Vec::new(&env));
        
        // Logika Auto-Increment ID
        let mut id: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        id += 1;

        let new_review = MovieReview {
            id,
            movie_id,
            title,
            rating,
            review,
            timestamp: env.ledger().timestamp(), // Mengambil waktu ledger saat ini
        };

        reviews.push_back(new_review);
        
        // Simpan data dan update counter
        env.storage().instance().set(&REVIEWS, &reviews);
        env.storage().instance().set(&COUNTER, &id);

        id
    }

    // 2. Fungsi untuk mengambil semua riwayat tontonan
    pub fn get_all_reviews(env: Env) -> Vec<MovieReview> {
        env.storage().instance().get(&REVIEWS).unwrap_or(Vec::new(&env))
    }

    // 3. Fungsi untuk memperbarui rating (jika selera berubah)
    pub fn update_rating(env: Env, id: u32, new_rating: u32) -> bool {
        let mut reviews: Vec<MovieReview> = env.storage().instance().get(&REVIEWS).unwrap_or(Vec::new(&env));

        for i in 0..reviews.len() {
            let mut rev = reviews.get(i).unwrap();
            if rev.id == id {
                rev.rating = new_rating;
                reviews.set(i, rev);
                env.storage().instance().set(&REVIEWS, &reviews);
                return true;
            }
        }
        false
    }

    // 4. Fungsi untuk menghapus ulasan dari diary
    pub fn remove_review(env: Env, id: u32) -> bool {
        let mut reviews: Vec<MovieReview> = env.storage().instance().get(&REVIEWS).unwrap_or(Vec::new(&env));

        for i in 0..reviews.len() {
            if reviews.get(i).unwrap().id == id {
                reviews.remove(i);
                env.storage().instance().set(&REVIEWS, &reviews);
                return true;
            }
        }
        false
    }
}