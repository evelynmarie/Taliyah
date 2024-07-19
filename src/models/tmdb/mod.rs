use chrono::prelude::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Movie {
    pub adult: bool,                                  // Whether or not the movie has an adult rating.
    pub belongs_to_collection: Option<Collection>,    // The movie's collection, if applicable.
    pub backdrop_path: Option<String>,                // The URL of the movie's backdrop.
    pub budget: u64,                                  // The movie's total budget.
    pub genres: Vec<Genre>,                           // Genres that apply to the movie.
    pub homepage: Option<String>,                     // The movie's website.
    pub id: u64,                                      // The movie's The Movie Database identifier.
    pub imdb_id: Option<String>,                      // The movie's IMDb identifier.
    pub original_language: String,                    // The movie's original language.
    pub original_title: String,                       // The movie's original title.
    pub overview: Option<String>,                     // The movie's overview / description.
    pub popularity: f64,                              // The movie's popularity.
    pub poster_path: Option<String>,                  // The movie's poster URL.
    pub production_companies: Vec<ProductionCompany>, // The movie's production companies.
    pub production_countries: Vec<ProductionCountry>, // The movie's production countries.
    pub release_date: Option<NaiveDate>,              // The movie's release date.
    pub revenue: u64,                                 // The movie's total amount of revenue.
    pub runtime: Option<u64>,                         // The movie's runtime duration, in minutes.
    pub status: String,                               // The movie's current status as listed on The Movie Database.
    pub tagline: Option<String>,                      // The movie's tagline.
    pub title: String,                                // The movie's title.
    pub video: bool,                                  // Whether or not this movie has a video available.
    pub vote_average: f64,                            // The movie's average user score on The Movie Database.
    pub vote_count: f64                               // The movie's total amount of votes on The Movie Database.
}

#[derive(Deserialize)]
pub struct Collection {
    pub id: u64,               // The ID of the collection.
    pub name: String,          // The name of the collection.
    pub poster_path: String,   // The poster of the collection.
    pub backdrop_path: String  // the backdrop of the collection.
}

#[derive(Deserialize)]
pub struct Genre {
    pub id: u64,      // The genre's ID.
    pub name: String  // The genre's name.
}

#[derive(Deserialize)]
pub struct ProductionCompany {
    pub name: String,           // The friendly name of the production company.
    pub id: u64,                // The ID of the production company on The Movie Database.
    pub origin_country: String  // The country of origin of the production company.
}

#[derive(Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String, // The ISO standard shortcode of the production country.
    pub name: String        // The friendly name of the production country.
}

#[derive(Deserialize)]
#[rustfmt::skip]
pub struct Show {
    pub backdrop_path: Option<String>,             // The show's backdrop path.
    pub created_by: Vec<CreatedBy>,                // The show's creators.
    pub episode_run_time: Vec<i64>,                // An array containing the show's episode runtimes.
    pub first_air_date: NaiveDate,                 // The date the show first aired.
    pub genres: Vec<Genre>,                        // The genres that the show is in.
    pub homepage: String,                          // The show's homepage.
    pub id: i64,                                   // The show's id on The Movie Database.
    pub in_production: bool,                       // Whether or not the show is currently in production.
    pub languages: Vec<String>,                    // The show's available languages.
    pub last_air_date: NaiveDate,                  // When the show last aired an episode.
    pub last_episode_to_air: EpisodeToAir,         // The show's last aired episode.
    pub name: String,                              // The name of the show.
    pub next_episode_to_air: Option<EpisodeToAir>, // The show's next scheduled episode.
    pub networks: Vec<NetworkOrStudio>,            // The networks or services that air the show.
    pub number_of_episodes: i64,                   // The total number of episodes the show has aired.
    pub number_of_seasons: i64,                    // The total number of seasons the show has released.
    pub origin_country: Vec<String>,               // The country where the show originated.
    pub original_language: String,                 // The original language of the show.
    pub original_name: String,                     // The show's original name.
    pub overview: String,                          // The show's overview.
    pub popularity: f64,                           // An integer containing the show's popularity value.
    pub poster_path: Option<String>,               // The show's poster path.
    #[serde(rename = "production_companies")]
    pub studios: Vec<NetworkOrStudio>,             // The studios that produce and manage the show.
    pub seasons: Vec<Season>,                      // A vector array containing information on the show's individual seasons.
    pub spoken_languages: Vec<Language>,           // A vector array containing information about the show's spoken languages.
    pub status: String,                            // The status of the show; can be Returning Series, Cancelled, or Ended.
    pub tagline: String,                           // The show's tagline.
    #[serde(rename = "type")]
    pub format: String,                            // The format of the show; can be Scripted, News, or Unscripted.
    pub vote_average: f64,                         // The show's average user score on The Movie Database.
    pub vote_count: i64,                           // The show's total amount of user votes on The Movie Database.
    pub external_ids: ExternalId                   // The external IDs associated with the show, e.g. the external IMDb ID.
}

#[derive(Deserialize)]
pub struct CreatedBy {
    pub id: i64,                      // The ID associated with the given creator.
    pub credit_id: String,            // The credit ID associated with the given creator.
    pub name: String,                 // The name of the given creator.
    pub gender: Option<i64>,          // The (optional) gender of the given creator.
    pub profile_path: Option<String>  // The (optional) profile path of the given creator.
}

#[derive(Deserialize)]
pub struct EpisodeToAir {
    pub air_date: Option<NaiveDate>, // The episode's air date.
    pub episode_number: i64,         // The number of the episode.
    pub id: i64,                     // The episode's TMDb ID.
    pub name: String,                // The name of the episode.
    pub overview: String,            // The episode's overview / synopsis.
    pub production_code: String,     // The episode's production code.
    pub season_number: i64,          // The season associated with the episode.
    pub still_path: Option<String>,  // The episode's still path.
    pub vote_average: f64,           // The episode's average user score on The Movie Database.
    pub vote_count: i64              // The total amount of votes for the episode.
}

#[derive(Deserialize)]
pub struct NetworkOrStudio {
    pub name: String,                   // The name of the studio.
    pub id: i64,                        // The ID associated with the studio.
    pub logo_path: Option<String>,      // The studio's logo path.
    pub origin_country: Option<String>  // The country where the studio originated.
}

#[derive(Deserialize)]
pub struct Season {
    pub air_date: Option<NaiveDate>, // The premiere date of the season.
    pub episode_count: i64,          // The total amount of episodes in the season.
    pub id: i64,                     // The season's TMDb identifier.
    pub name: String,                // The name of the season. Typically just "Season <number>".
    pub overview: Option<String>,    // An overview / synopsis about the season.
    pub poster_path: Option<String>, // The poster path of the season.
    pub season_number: i64           // The season's numerical number.
}

#[derive(Deserialize)]
pub struct Language {
    pub english_name: String, // The name of the given language, in English.
    pub iso_639_1: String,    // The ISO 639-1 identifier associated with the language.
    pub name: String          // The native name associated with the language.
}

#[derive(Deserialize)]
pub struct ExternalId {
    pub imdb_id: Option<String>,      // The show's IMDb identifier.
    pub freebase_mid: Option<String>, // The show's Freebase MID.
    pub freebase_id: Option<String>,  // The show's freebase ID.
    pub tvdb_id: Option<i64>,         // The show's TVDb identifier.
    pub tvrage_id: Option<i64>,       // The show's TVRage identifier.
    pub facebook_id: Option<String>,  // The ID of the show's Facebook page.
    pub instagram_id: Option<String>, // The ID of the show's Instagram profile.
    pub twitter_id: Option<String>,   // The ID of the show's Twitter profile.
    pub id: Option<i64>               // The show's The Movie Database identifier.
}
