//! Contains the `PlaceType` enum and its associated traits. It specifies the
//! types or categories of a place. For example, a returned place could be a
//! "country" (as in a nation) or it could be a "shopping mall."

use crate::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// This specifies the types or categories of a place. For example, a returned
/// location could be a "country" (as in a nation) or it could be a "shopping
/// mall." Also, a requested place could be a "locality" (a city) or a
/// "street_address" This type helps define the data that is being returned or
/// sought. See [Place
/// Types](https://developers.google.com/places/web-service/supported_types)
/// for more information.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PlaceType {
    // [Table 1: Place types](https://developers.google.com/places/web-service/supported_types#table1)
    // The types that are supported for place searches, and can be returned with Place details results, and as part of autocomplete place predictions.
    #[serde(alias = "accounting")]
    Accounting,
    /// Indicates an airport.
    #[serde(alias = "airport")]
    Airport,
    #[serde(alias = "amusement_park")]
    AmusementPark,
    #[serde(alias = "aquarium")]
    Aquarium,
    #[serde(alias = "art_gallery")]
    ArtGallery,
    #[serde(alias = "atm")]
    Atm,
    #[serde(alias = "bakery")]
    Bakery,
    #[serde(alias = "bank")]
    Bank,
    #[serde(alias = "bar")]
    Bar,
    #[serde(alias = "beauty_salon")]
    BeautySalon,
    #[serde(alias = "bicycle_store")]
    BicycleStore,
    #[serde(alias = "book_store")]
    BookStore,
    #[serde(alias = "bowling_alley")]
    BowlingAlley,
    #[serde(alias = "bus_station")]
    BusStation,
    #[serde(alias = "cafe")]
    Cafe,
    #[serde(alias = "campground")]
    Campground,
    #[serde(alias = "car_dealer")]
    CarDealer,
    #[serde(alias = "car_rental")]
    CarRental,
    #[serde(alias = "car_repair")]
    CarRepair,
    #[serde(alias = "car_wash")]
    CarWash,
    #[serde(alias = "casino")]
    Casino,
    #[serde(alias = "cemetery")]
    Cemetery,
    #[serde(alias = "church")]
    Church,
    #[serde(alias = "city_hall")]
    CityHall,
    #[serde(alias = "clothing_store")]
    ClothingStore,
    #[serde(alias = "convenience_store")]
    ConvenienceStore,
    #[serde(alias = "courthouse")]
    Courthouse,
    #[serde(alias = "dentist")]
    Dentist,
    #[serde(alias = "department_store")]
    DepartmentStore,
    #[serde(alias = "doctor")]
    Doctor,
    #[serde(alias = "drugstore")]
    DrugStore,
    #[serde(alias = "electrician")]
    Electrician,
    #[serde(alias = "electronics_store")]
    ElectronicsStore,
    #[serde(alias = "embassy")]
    Embassy,
    #[serde(alias = "fire_station")]
    FireStation,
    #[serde(alias = "florist")]
    Florist,
    #[serde(alias = "funeral_home")]
    FuneralHome,
    #[serde(alias = "furniture_store")]
    FurnitureStore,
    #[serde(alias = "gas_station")]
    GasStation,
    #[serde(alias = "grocery_or_supermarket")]
    GroceryOrSupermarket,
    #[serde(alias = "gym")]
    Gym,
    #[serde(alias = "hair_care")]
    HairCare,
    #[serde(alias = "hardware_store")]
    HardwareStore,
    #[serde(alias = "hindu_temple")]
    HinduTemple,
    #[serde(alias = "home_goods_store")]
    HomeGoodsStore,
    #[serde(alias = "hospital")]
    Hospital,
    #[serde(alias = "insurance_agency")]
    InsuranceAgency,
    #[serde(alias = "jewelry_store")]
    JewelryStore,
    #[serde(alias = "laundry")]
    Laundry,
    #[serde(alias = "lawyer")]
    Lawyer,
    #[serde(alias = "library")]
    Library,
    #[serde(alias = "light_rail_station")]
    LightRailStation,
    #[serde(alias = "liquor_store")]
    LiquorStore,
    #[serde(alias = "local_government_office")]
    LocalGovernmentOffice,
    #[serde(alias = "locksmith")]
    Locksmith,
    #[serde(alias = "lodging")]
    Lodging,
    #[serde(alias = "meal_delivery")]
    MealDelivery,
    #[serde(alias = "meal_takeaway")]
    MealTakeaway,
    #[serde(alias = "mosque")]
    Mosque,
    #[serde(alias = "movie_rental")]
    MovieRental,
    #[serde(alias = "movie_theater")]
    MovieTheater,
    #[serde(alias = "moving_company")]
    MovingCompany,
    #[serde(alias = "museum")]
    Museum,
    #[serde(alias = "night_club")]
    NightClub,
    #[serde(alias = "painter")]
    Painter,
    /// Indicates a named park.
    #[serde(alias = "park")]
    Park,
    #[serde(alias = "parking")]
    Parking,
    #[serde(alias = "pet_store")]
    PetStore,
    #[serde(alias = "pharmacy")]
    Pharmacy,
    #[serde(alias = "physiotherapist")]
    Physiotherapist,
    #[serde(alias = "plumber")]
    Plumber,
    #[serde(alias = "plus_code")]
    PlusCode,
    #[serde(alias = "police")]
    Police,
    #[serde(alias = "post_office")]
    PostOffice,
    #[serde(alias = "primary_school")]
    PrimarySchool,
    #[serde(alias = "real_estate_agency")]
    RealEstateAgency,
    #[serde(alias = "restaurant")]
    Restaurant,
    #[serde(alias = "roofing_contractor")]
    RoofingContractor,
    #[serde(alias = "rv_park")]
    RvPark,
    #[serde(alias = "school")]
    School,
    #[serde(alias = "secondary_school")]
    SecondarySchool,
    #[serde(alias = "shoe_store")]
    ShoeStore,
    #[serde(alias = "shopping_mall")]
    ShoppingMall,
    #[serde(alias = "spa")]
    Spa,
    #[serde(alias = "stadium")]
    Stadium,
    #[serde(alias = "storage")]
    Storage,
    #[serde(alias = "store")]
    Store,
    #[serde(alias = "subway_station")]
    SubwayStation,
    #[serde(alias = "supermarket")]
    Supermarket,
    #[serde(alias = "synagogue")]
    Synagogue,
    #[serde(alias = "taxi_stand")]
    TaxiStand,
    #[serde(alias = "tourist_attraction")]
    TouristAttraction,
    #[serde(alias = "train_station")]
    TrainStation,
    #[serde(alias = "transit_station")]
    TransitStation,
    #[serde(alias = "travel_agency")]
    TravelAgency,
    #[serde(alias = "university")]
    University,
    #[serde(alias = "veterinary_care")]
    VeterinaryCare,
    #[serde(alias = "zoo")]
    Zoo,
    // [Table 2: Additional types returned by the Places service](https://developers.google.com/places/web-service/supported_types#table2)
    // Additional types that can be returned with Place details results, and as part of autocomplete place predictions.
    // Note: The types below are *not supported* in the `type` filter of a place search.
    /// Indicates a first-order civil entity below the country level. Within the
    /// United States, these administrative levels are states. Not all nations
    /// exhibit these administrative levels. In most cases,
    /// `AdministrativeAreaLevel1` short names will closely match ISO 3166-2
    /// subdivisions and other widely circulated lists; however this is not
    /// guaranteed as our geocoding results are based on a variety of signals
    /// and location data.
    #[serde(alias = "administrative_area_level_1")]
    AdministrativeAreaLevel1,
    /// Indicates a second-order civil entity below the country level. Within
    /// the United States, these administrative levels are counties. Not all
    /// nations exhibit these administrative levels.
    #[serde(alias = "administrative_area_level_2")]
    AdministrativeAreaLevel2,
    /// Indicates a third-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    #[serde(alias = "administrative_area_level_3")]
    AdministrativeAreaLevel3,
    /// Indicates a fourth-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    #[serde(alias = "administrative_area_level_4")]
    AdministrativeAreaLevel4,
    /// Indicates a fifth-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    #[serde(alias = "administrative_area_level_5")]
    AdministrativeAreaLevel5,
    #[serde(alias = "archipelago")]
    Archipelago,
    /// Indicates a commonly-used alternative name for the entity.
    #[serde(alias = "colloquial_area")]
    ColloquialArea,
    #[serde(alias = "continent")]
    Continent,
    /// Indicates the national political entity, and is typically the highest
    /// order type returned by the Geocoder.
    #[serde(alias = "country")]
    Country,
    #[serde(alias = "establishment")]
    Establishment,
    #[serde(alias = "finance")]
    Finance,
    #[serde(alias = "floor")]
    Floor,
    #[serde(alias = "food")]
    Food,
    #[serde(alias = "general_contractor")]
    GeneralContractor,
    #[serde(alias = "geocode")]
    Geocode,
    #[serde(alias = "health")]
    Health,
    /// Indicates a major intersection, usually of two major roads.
    #[serde(alias = "intersection")]
    Intersection,
    /// Indicates an incorporated city or town political entity.
    #[serde(alias = "locality")]
    Locality,
    /// Indicates a prominent natural feature.
    #[serde(alias = "natural_feature")]
    NaturalFeature,
    /// Indicates a named neighborhood.
    #[serde(alias = "neighborhood")]
    Neighborhood,
    #[serde(alias = "place_of_worship")]
    PlaceOfWorship,
    /// Indicates a named point of interest. Typically, these "POI"s are
    /// prominent local entities that don't easily fit in another category, such
    /// as "Empire State Building" or "Eiffel Tower".
    #[serde(alias = "point_of_interest")]
    PointOfInterest,
    /// Indicates a political entity. Usually, this type indicates a polygon of
    /// some civil administration.
    #[serde(alias = "political")]
    Political,
    #[serde(alias = "post_box")]
    PostBox,
    /// Indicates a postal code as used to address postal mail within the
    /// country.
    #[serde(alias = "postal_code")]
    PostalCode,
    #[serde(alias = "postal_code_prefix")]
    PostalCodePrefix,
    #[serde(alias = "postal_code_suffix")]
    PostalCodeSuffix,
    #[serde(alias = "postal_town")]
    PostalTown,
    /// Indicates a named location, usually a building or collection of
    /// buildings with a common name.
    #[serde(alias = "premise")]
    Premise,
    #[serde(alias = "room")]
    Room,
    /// Indicates a named route (such as "US 101").
    #[serde(alias = "route")]
    Route,
    /// Indicates a precise street address.
    #[serde(alias = "street_address")]
    StreetAddress,
    #[serde(alias = "street_number")]
    StreetNumber,
    /// Indicates a first-order civil entity below a locality. For some
    /// locations may receive one of the additional types: `SublocalityLevel1`
    /// to `SublocalityLevel5`. Each sublocality level is a civil entity. Larger
    /// numbers indicate a smaller geographic area.
    #[serde(alias = "sublocality")]
    Sublocality,
    #[serde(alias = "sublocality_level_1")]
    SublocalityLevel1,
    #[serde(alias = "sublocality_level_2")]
    SublocalityLevel2,
    #[serde(alias = "sublocality_level_3")]
    SublocalityLevel3,
    #[serde(alias = "sublocality_level_4")]
    SublocalityLevel4,
    #[serde(alias = "sublocality_level_5")]
    SublocalityLevel5,
    /// Indicates a first-order entity below a named location, usually a
    /// singular building within a collection of buildings with a common name.
    #[serde(alias = "subpremise")]
    Subpremise,
    #[serde(alias = "town_square")]
    TownSquare,
    // [Table 3: Types supported in place autocomplete requests](https://developers.google.com/places/web-service/supported_types#table3)
    // Types you can use in place autocomplete requests.
    // #[serde(alias = "geocode")]
    // Geocode,
    #[serde(alias = "address")]
    Address,
    // #[serde(alias = "establishment")]
    // Establishment,
    #[serde(alias = "regions")]
    Regions,
    #[serde(alias = "cities")]
    Cities,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for PlaceType {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match PlaceType::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&PlaceType> for String {
    /// Converts a `PlaceType` enum to a `String` that contains a [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn from(place_type: &PlaceType) -> String {
        match place_type {
            PlaceType::Accounting => String::from("accounting"),
            PlaceType::Airport => String::from("airport"),
            PlaceType::AmusementPark => String::from("amusement_park"),
            PlaceType::Aquarium => String::from("aquarium"),
            PlaceType::ArtGallery => String::from("art_gallery"),
            PlaceType::Atm => String::from("atm"),
            PlaceType::Bakery => String::from("bakery"),
            PlaceType::Bank => String::from("bank"),
            PlaceType::Bar => String::from("bar"),
            PlaceType::BeautySalon => String::from("beauty_salon"),
            PlaceType::BicycleStore => String::from("bicycle_store"),
            PlaceType::BookStore => String::from("book_store"),
            PlaceType::BowlingAlley => String::from("bowling_alley"),
            PlaceType::BusStation => String::from("bus_station"),
            PlaceType::Cafe => String::from("cafe"),
            PlaceType::Campground => String::from("campground"),
            PlaceType::CarDealer => String::from("car_dealer"),
            PlaceType::CarRental => String::from("car_rental"),
            PlaceType::CarRepair => String::from("car_repair"),
            PlaceType::CarWash => String::from("car_wash"),
            PlaceType::Casino => String::from("casino"),
            PlaceType::Cemetery => String::from("cemetery"),
            PlaceType::Church => String::from("church"),
            PlaceType::CityHall => String::from("city_hall"),
            PlaceType::ClothingStore => String::from("clothing_store"),
            PlaceType::ConvenienceStore => String::from("convenience_store"),
            PlaceType::Courthouse => String::from("courthouse"),
            PlaceType::Dentist => String::from("dentist"),
            PlaceType::DepartmentStore => String::from("department_store"),
            PlaceType::Doctor => String::from("doctor"),
            PlaceType::DrugStore => String::from("drugstore"),
            PlaceType::Electrician => String::from("electrician"),
            PlaceType::ElectronicsStore => String::from("electronics_store"),
            PlaceType::Embassy => String::from("embassy"),
            PlaceType::FireStation => String::from("fire_station"),
            PlaceType::Florist => String::from("florist"),
            PlaceType::FuneralHome => String::from("funeral_home"),
            PlaceType::FurnitureStore => String::from("furniture_store"),
            PlaceType::GasStation => String::from("gas_station"),
            PlaceType::GroceryOrSupermarket => String::from("grocery_or_supermarket"),
            PlaceType::Gym => String::from("gym"),
            PlaceType::HairCare => String::from("hair_care"),
            PlaceType::HardwareStore => String::from("hardware_store"),
            PlaceType::HinduTemple => String::from("hindu_temple"),
            PlaceType::HomeGoodsStore => String::from("home_goods_store"),
            PlaceType::Hospital => String::from("hospital"),
            PlaceType::InsuranceAgency => String::from("insurance_agency"),
            PlaceType::JewelryStore => String::from("jewelry_store"),
            PlaceType::Laundry => String::from("laundry"),
            PlaceType::Lawyer => String::from("lawyer"),
            PlaceType::Library => String::from("library"),
            PlaceType::LightRailStation => String::from("light_rail_station"),
            PlaceType::LiquorStore => String::from("liquor_store"),
            PlaceType::LocalGovernmentOffice => String::from("local_government_office"),
            PlaceType::Locksmith => String::from("locksmith"),
            PlaceType::Lodging => String::from("lodging"),
            PlaceType::MealDelivery => String::from("meal_delivery"),
            PlaceType::MealTakeaway => String::from("meal_takeaway"),
            PlaceType::Mosque => String::from("mosque"),
            PlaceType::MovieRental => String::from("movie_rental"),
            PlaceType::MovieTheater => String::from("movie_theater"),
            PlaceType::MovingCompany => String::from("moving_company"),
            PlaceType::Museum => String::from("museum"),
            PlaceType::NightClub => String::from("night_club"),
            PlaceType::Painter => String::from("painter"),
            PlaceType::Park => String::from("park"),
            PlaceType::Parking => String::from("parking"),
            PlaceType::PetStore => String::from("pet_store"),
            PlaceType::Pharmacy => String::from("pharmacy"),
            PlaceType::Physiotherapist => String::from("physiotherapist"),
            PlaceType::Plumber => String::from("plumber"),
            PlaceType::PlusCode => String::from("plus_code"),
            PlaceType::Police => String::from("police"),
            PlaceType::PostOffice => String::from("post_office"),
            PlaceType::PrimarySchool => String::from("primary_school"),
            PlaceType::RealEstateAgency => String::from("real_estate_agency"),
            PlaceType::Restaurant => String::from("restaurant"),
            PlaceType::RoofingContractor => String::from("roofing_contractor"),
            PlaceType::RvPark => String::from("rv_park"),
            PlaceType::School => String::from("school"),
            PlaceType::SecondarySchool => String::from("secondary_school"),
            PlaceType::ShoeStore => String::from("shoe_store"),
            PlaceType::ShoppingMall => String::from("shopping_mall"),
            PlaceType::Spa => String::from("spa"),
            PlaceType::Stadium => String::from("stadium"),
            PlaceType::Storage => String::from("storage"),
            PlaceType::Store => String::from("store"),
            PlaceType::SubwayStation => String::from("subway_station"),
            PlaceType::Supermarket => String::from("supermarket"),
            PlaceType::Synagogue => String::from("synagogue"),
            PlaceType::TaxiStand => String::from("taxi_stand"),
            PlaceType::TouristAttraction => String::from("tourist_attraction"),
            PlaceType::TrainStation => String::from("train_station"),
            PlaceType::TransitStation => String::from("transit_station"),
            PlaceType::TravelAgency => String::from("travel_agency"),
            PlaceType::University => String::from("university"),
            PlaceType::VeterinaryCare => String::from("veterinary_care"),
            PlaceType::Zoo => String::from("zoo"),
            PlaceType::AdministrativeAreaLevel1 => String::from("administrative_area_level_1"),
            PlaceType::AdministrativeAreaLevel2 => String::from("administrative_area_level_2"),
            PlaceType::AdministrativeAreaLevel3 => String::from("administrative_area_level_3"),
            PlaceType::AdministrativeAreaLevel4 => String::from("administrative_area_level_4"),
            PlaceType::AdministrativeAreaLevel5 => String::from("administrative_area_level_5"),
            PlaceType::Archipelago => String::from("archipelago"),
            PlaceType::ColloquialArea => String::from("colloquial_area"),
            PlaceType::Continent => String::from("continent"),
            PlaceType::Country => String::from("country"),
            PlaceType::Establishment => String::from("establishment"),
            PlaceType::Finance => String::from("finance"),
            PlaceType::Floor => String::from("floor"),
            PlaceType::Food => String::from("food"),
            PlaceType::GeneralContractor => String::from("general_contractor"),
            PlaceType::Geocode => String::from("geocode"),
            PlaceType::Health => String::from("health"),
            PlaceType::Intersection => String::from("intersection"),
            PlaceType::Locality => String::from("locality"),
            PlaceType::NaturalFeature => String::from("natural_feature"),
            PlaceType::Neighborhood => String::from("neighborhood"),
            PlaceType::PlaceOfWorship => String::from("place_of_worship"),
            PlaceType::PointOfInterest => String::from("point_of_interest"),
            PlaceType::Political => String::from("political"),
            PlaceType::PostBox => String::from("post_box"),
            PlaceType::PostalCode => String::from("postal_code"),
            PlaceType::PostalCodePrefix => String::from("postal_code_prefix"),
            PlaceType::PostalCodeSuffix => String::from("postal_code_suffix"),
            PlaceType::PostalTown => String::from("postal_town"),
            PlaceType::Premise => String::from("premise"),
            PlaceType::Room => String::from("room"),
            PlaceType::Route => String::from("route"),
            PlaceType::StreetAddress => String::from("street_address"),
            PlaceType::StreetNumber => String::from("street_number"),
            PlaceType::Sublocality => String::from("sublocality"),
            PlaceType::SublocalityLevel1 => String::from("sublocality_level_1"),
            PlaceType::SublocalityLevel2 => String::from("sublocality_level_2"),
            PlaceType::SublocalityLevel3 => String::from("sublocality_level_3"),
            PlaceType::SublocalityLevel4 => String::from("sublocality_level_4"),
            PlaceType::SublocalityLevel5 => String::from("sublocality_level_5"),
            PlaceType::Subpremise => String::from("subpremise"),
            PlaceType::TownSquare => String::from("town_square"),
            PlaceType::Address => String::from("address"),
            PlaceType::Regions => String::from("regions"),
            PlaceType::Cities => String::from("cities"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static PLACE_TYPES_BY_CODE: phf::Map<&'static str, PlaceType> = phf_map! {
    "accounting" => PlaceType::Accounting,
    "airport" => PlaceType::Airport,
    "amusement_park" => PlaceType::AmusementPark,
    "aquarium" => PlaceType::Aquarium,
    "art_gallery" => PlaceType::ArtGallery,
    "atm" => PlaceType::Atm,
    "bakery" => PlaceType::Bakery,
    "bank" => PlaceType::Bank,
    "bar" => PlaceType::Bar,
    "beauty_salon" => PlaceType::BeautySalon,
    "bicycle_store" => PlaceType::BicycleStore,
    "book_store" => PlaceType::BookStore,
    "bowling_alley" => PlaceType::BowlingAlley,
    "bus_station" => PlaceType::BusStation,
    "cafe" => PlaceType::Cafe,
    "campground" => PlaceType::Campground,
    "car_dealer" => PlaceType::CarDealer,
    "car_rental" => PlaceType::CarRental,
    "car_repair" => PlaceType::CarRepair,
    "car_wash" => PlaceType::CarWash,
    "casino" => PlaceType::Casino,
    "cemetery" => PlaceType::Cemetery,
    "church" => PlaceType::Church,
    "city_hall" => PlaceType::CityHall,
    "clothing_store" => PlaceType::ClothingStore,
    "convenience_store" => PlaceType::ConvenienceStore,
    "courthouse" => PlaceType::Courthouse,
    "dentist" => PlaceType::Dentist,
    "department_store" => PlaceType::DepartmentStore,
    "doctor" => PlaceType::Doctor,
    "drugstore" => PlaceType::DrugStore,
    "electrician" => PlaceType::Electrician,
    "electronics_store" => PlaceType::ElectronicsStore,
    "embassy" => PlaceType::Embassy,
    "fire_station" => PlaceType::FireStation,
    "florist" => PlaceType::Florist,
    "funeral_home" => PlaceType::FuneralHome,
    "furniture_store" => PlaceType::FurnitureStore,
    "gas_station" => PlaceType::GasStation,
    "grocery_or_supermarket" => PlaceType::GroceryOrSupermarket,
    "gym" => PlaceType::Gym,
    "hair_care" => PlaceType::HairCare,
    "hardware_store" => PlaceType::HardwareStore,
    "hindu_temple" => PlaceType::HinduTemple,
    "home_goods_store" => PlaceType::HomeGoodsStore,
    "hospital" => PlaceType::Hospital,
    "insurance_agency" => PlaceType::InsuranceAgency,
    "jewelry_store" => PlaceType::JewelryStore,
    "laundry" => PlaceType::Laundry,
    "lawyer" => PlaceType::Lawyer,
    "library" => PlaceType::Library,
    "light_rail_station" => PlaceType::LightRailStation,
    "liquor_store" => PlaceType::LiquorStore,
    "local_government_office" => PlaceType::LocalGovernmentOffice,
    "locksmith" => PlaceType::Locksmith,
    "lodging" => PlaceType::Lodging,
    "meal_delivery" => PlaceType::MealDelivery,
    "meal_takeaway" => PlaceType::MealTakeaway,
    "mosque" => PlaceType::Mosque,
    "movie_rental" => PlaceType::MovieRental,
    "movie_theater" => PlaceType::MovieTheater,
    "moving_company" => PlaceType::MovingCompany,
    "museum" => PlaceType::Museum,
    "night_club" => PlaceType::NightClub,
    "painter" => PlaceType::Painter,
    "park" => PlaceType::Park,
    "parking" => PlaceType::Parking,
    "pet_store" => PlaceType::PetStore,
    "pharmacy" => PlaceType::Pharmacy,
    "physiotherapist" => PlaceType::Physiotherapist,
    "plumber" => PlaceType::Plumber,
    "plus_code" => PlaceType::PlusCode,
    "police" => PlaceType::Police,
    "post_office" => PlaceType::PostOffice,
    "primary_school" => PlaceType::PrimarySchool,
    "real_estate_agency" => PlaceType::RealEstateAgency,
    "restaurant" => PlaceType::Restaurant,
    "roofing_contractor" => PlaceType::RoofingContractor,
    "rv_park" => PlaceType::RvPark,
    "school" => PlaceType::School,
    "secondary_school" => PlaceType::SecondarySchool,
    "shoe_store" => PlaceType::ShoeStore,
    "shopping_mall" => PlaceType::ShoppingMall,
    "spa" => PlaceType::Spa,
    "stadium" => PlaceType::Stadium,
    "storage" => PlaceType::Storage,
    "store" => PlaceType::Store,
    "subway_station" => PlaceType::SubwayStation,
    "supermarket" => PlaceType::Supermarket,
    "synagogue" => PlaceType::Synagogue,
    "taxi_stand" => PlaceType::TaxiStand,
    "tourist_attraction" => PlaceType::TouristAttraction,
    "train_station" => PlaceType::TrainStation,
    "transit_station" => PlaceType::TransitStation,
    "travel_agency" => PlaceType::TravelAgency,
    "university" => PlaceType::University,
    "veterinary_care" => PlaceType::VeterinaryCare,
    "zoo" => PlaceType::Zoo,
    "administrative_area_level_1" => PlaceType::AdministrativeAreaLevel1,
    "administrative_area_level_2" => PlaceType::AdministrativeAreaLevel2,
    "administrative_area_level_3" => PlaceType::AdministrativeAreaLevel3,
    "administrative_area_level_4" => PlaceType::AdministrativeAreaLevel4,
    "administrative_area_level_5" => PlaceType::AdministrativeAreaLevel5,
    "archipelago" => PlaceType::Archipelago,
    "colloquial_area" => PlaceType::ColloquialArea,
    "continent" => PlaceType::Continent,
    "country" => PlaceType::Country,
    "establishment" => PlaceType::Establishment,
    "finance" => PlaceType::Finance,
    "floor" => PlaceType::Floor,
    "food" => PlaceType::Food,
    "general_contractor" => PlaceType::GeneralContractor,
    "geocode" => PlaceType::Geocode,
    "health" => PlaceType::Health,
    "intersection" => PlaceType::Intersection,
    "locality" => PlaceType::Locality,
    "natural_feature" => PlaceType::NaturalFeature,
    "neighborhood" => PlaceType::Neighborhood,
    "place_of_worship" => PlaceType::PlaceOfWorship,
    "point_of_interest" => PlaceType::PointOfInterest,
    "political" => PlaceType::Political,
    "post_box" => PlaceType::PostBox,
    "postal_code" => PlaceType::PostalCode,
    "postal_code_prefix" => PlaceType::PostalCodePrefix,
    "postal_code_suffix" => PlaceType::PostalCodeSuffix,
    "postal_town" => PlaceType::PostalTown,
    "premise" => PlaceType::Premise,
    "room" => PlaceType::Room,
    "route" => PlaceType::Route,
    "street_address" => PlaceType::StreetAddress,
    "street_number" => PlaceType::StreetNumber,
    "sublocality" => PlaceType::Sublocality,
    "sublocality_level_1" => PlaceType::SublocalityLevel1,
    "sublocality_level_2" => PlaceType::SublocalityLevel2,
    "sublocality_level_3" => PlaceType::SublocalityLevel3,
    "sublocality_level_4" => PlaceType::SublocalityLevel4,
    "sublocality_level_5" => PlaceType::SublocalityLevel5,
    "subpremise" => PlaceType::Subpremise,
    "town_square" => PlaceType::TownSquare,
    "address" => PlaceType::Address,
    "regions" => PlaceType::Regions,
    "cities" => PlaceType::Cities,
};

impl std::convert::TryFrom<&str> for PlaceType {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;
    /// Gets a `PlaceType` enum from a `String` that contains a supported [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn try_from(place_type_code: &str) -> Result<Self, Self::Error> {
        PLACE_TYPES_BY_CODE
            .get(place_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidPlaceTypeCode(place_type_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for PlaceType {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Err = crate::error::Error;
    /// Gets a `PlaceType` enum from a `String` that contains a supported [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn from_str(place_type_code: &str) -> Result<Self, Self::Err> {
        PLACE_TYPES_BY_CODE
            .get(place_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidPlaceTypeCode(place_type_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for PlaceType {
    /// Returns a reasonable default variant for the `PlaceType` enum type.
    fn default() -> Self {
        PlaceType::Locality
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for PlaceType {
    /// Formats a `PlaceType` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlaceType::Accounting => write!(f, "Accounting"),
            PlaceType::Airport => write!(f, "Airport"),
            PlaceType::AmusementPark => write!(f, "Amusement Park"),
            PlaceType::Aquarium => write!(f, "Aquarium"),
            PlaceType::ArtGallery => write!(f, "Art Gallery"),
            PlaceType::Atm => write!(f, "ATM"),
            PlaceType::Bakery => write!(f, "Bakery"),
            PlaceType::Bank => write!(f, "Bank"),
            PlaceType::Bar => write!(f, "Bar"),
            PlaceType::BeautySalon => write!(f, "Beauty Salon"),
            PlaceType::BicycleStore => write!(f, "Bicycle Store"),
            PlaceType::BookStore => write!(f, "Book Store"),
            PlaceType::BowlingAlley => write!(f, "Bowling Alley"),
            PlaceType::BusStation => write!(f, "Bus Station"),
            PlaceType::Cafe => write!(f, "Café"),
            PlaceType::Campground => write!(f, "Campground"),
            PlaceType::CarDealer => write!(f, "Car Dealer"),
            PlaceType::CarRental => write!(f, "Car Rental"),
            PlaceType::CarRepair => write!(f, "Car Rrepair"),
            PlaceType::CarWash => write!(f, "Car Wash"),
            PlaceType::Casino => write!(f, "Casino"),
            PlaceType::Cemetery => write!(f, "Cemetery"),
            PlaceType::Church => write!(f, "Church"),
            PlaceType::CityHall => write!(f, "City Hall"),
            PlaceType::ClothingStore => write!(f, "Clothing Store"),
            PlaceType::ConvenienceStore => write!(f, "Convenience Store"),
            PlaceType::Courthouse => write!(f, "Courthouse"),
            PlaceType::Dentist => write!(f, "Dentist"),
            PlaceType::DepartmentStore => write!(f, "Department Store"),
            PlaceType::Doctor => write!(f, "Doctor"),
            PlaceType::DrugStore => write!(f, "Drug Store"),
            PlaceType::Electrician => write!(f, "Electrician"),
            PlaceType::ElectronicsStore => write!(f, "Electronics Store"),
            PlaceType::Embassy => write!(f, "Embassy"),
            PlaceType::FireStation => write!(f, "Fire Station"),
            PlaceType::Florist => write!(f, "Florist"),
            PlaceType::FuneralHome => write!(f, "Funeral Home"),
            PlaceType::FurnitureStore => write!(f, "Furniture Store"),
            PlaceType::GasStation => write!(f, "Gas Station"),
            PlaceType::GroceryOrSupermarket => write!(f, "Grocery or Supermarket"),
            PlaceType::Gym => write!(f, "Gym"),
            PlaceType::HairCare => write!(f, "Hair Care"),
            PlaceType::HardwareStore => write!(f, "Hardware Store"),
            PlaceType::HinduTemple => write!(f, "Hindu Temple"),
            PlaceType::HomeGoodsStore => write!(f, "Home Goods Store"),
            PlaceType::Hospital => write!(f, "Hospital"),
            PlaceType::InsuranceAgency => write!(f, "Insurance Agency"),
            PlaceType::JewelryStore => write!(f, "Jewelry Store"),
            PlaceType::Laundry => write!(f, "Laundry"),
            PlaceType::Lawyer => write!(f, "Lawyer"),
            PlaceType::Library => write!(f, "Library"),
            PlaceType::LightRailStation => write!(f, "Light Rail Station"),
            PlaceType::LiquorStore => write!(f, "Liquor Store"),
            PlaceType::LocalGovernmentOffice => write!(f, "Local Government Office"),
            PlaceType::Locksmith => write!(f, "Locksmith"),
            PlaceType::Lodging => write!(f, "Lodging"),
            PlaceType::MealDelivery => write!(f, "Meal Delivery"),
            PlaceType::MealTakeaway => write!(f, "Meal Takeaway"),
            PlaceType::Mosque => write!(f, "Mosque"),
            PlaceType::MovieRental => write!(f, "Movie Rental"),
            PlaceType::MovieTheater => write!(f, "Movie Theater"),
            PlaceType::MovingCompany => write!(f, "Moving Company"),
            PlaceType::Museum => write!(f, "Museum"),
            PlaceType::NightClub => write!(f, "Night Club"),
            PlaceType::Painter => write!(f, "Painter"),
            PlaceType::Park => write!(f, "Park"),
            PlaceType::Parking => write!(f, "Parking"),
            PlaceType::PetStore => write!(f, "Pet Store"),
            PlaceType::Pharmacy => write!(f, "Pharmacy"),
            PlaceType::Physiotherapist => write!(f, "Physiotherapist"),
            PlaceType::Plumber => write!(f, "Plumber"),
            PlaceType::PlusCode => write!(f, "Plus Code"),
            PlaceType::Police => write!(f, "Police"),
            PlaceType::PostOffice => write!(f, "Post Office"),
            PlaceType::PrimarySchool => write!(f, "Primary School"),
            PlaceType::RealEstateAgency => write!(f, "Real Estate Agency"),
            PlaceType::Restaurant => write!(f, "Restaurant"),
            PlaceType::RoofingContractor => write!(f, "Roofing Contractor"),
            PlaceType::RvPark => write!(f, "RV Park"),
            PlaceType::School => write!(f, "School"),
            PlaceType::SecondarySchool => write!(f, "Secondary School"),
            PlaceType::ShoeStore => write!(f, "Shoe Store"),
            PlaceType::ShoppingMall => write!(f, "Shopping Mall"),
            PlaceType::Spa => write!(f, "Spa"),
            PlaceType::Stadium => write!(f, "Stadium"),
            PlaceType::Storage => write!(f, "Storage"),
            PlaceType::Store => write!(f, "Store"),
            PlaceType::SubwayStation => write!(f, "Subway Station"),
            PlaceType::Supermarket => write!(f, "Supermarket"),
            PlaceType::Synagogue => write!(f, "Synagogue"),
            PlaceType::TaxiStand => write!(f, "Taxi Stand"),
            PlaceType::TouristAttraction => write!(f, "Tourist Attraction"),
            PlaceType::TrainStation => write!(f, "Train Station"),
            PlaceType::TransitStation => write!(f, "Transit Station"),
            PlaceType::TravelAgency => write!(f, "Travel Agency"),
            PlaceType::University => write!(f, "University"),
            PlaceType::VeterinaryCare => write!(f, "Veterinary Care"),
            PlaceType::Zoo => write!(f, "Zoo"),
            PlaceType::AdministrativeAreaLevel1 => write!(f, "Administrative Area Level 1"),
            PlaceType::AdministrativeAreaLevel2 => write!(f, "Administrative Area Level 2"),
            PlaceType::AdministrativeAreaLevel3 => write!(f, "Administrative Area Level 3"),
            PlaceType::AdministrativeAreaLevel4 => write!(f, "Administrative Area Level 4"),
            PlaceType::AdministrativeAreaLevel5 => write!(f, "Administrative Area Level 5"),
            PlaceType::Archipelago => write!(f, "Archipelago"),
            PlaceType::ColloquialArea => write!(f, "Colloquial Area"),
            PlaceType::Continent => write!(f, "Continent"),
            PlaceType::Country => write!(f, "Country"),
            PlaceType::Establishment => write!(f, "Establishment"),
            PlaceType::Finance => write!(f, "Finance"),
            PlaceType::Floor => write!(f, "Floor"),
            PlaceType::Food => write!(f, "Food"),
            PlaceType::GeneralContractor => write!(f, "General Contractor"),
            PlaceType::Geocode => write!(f, "Geocode"),
            PlaceType::Health => write!(f, "Health"),
            PlaceType::Intersection => write!(f, "Intersection"),
            PlaceType::Locality => write!(f, "Locality"),
            PlaceType::NaturalFeature => write!(f, "Natural Feature"),
            PlaceType::Neighborhood => write!(f, "Neighborhood"),
            PlaceType::PlaceOfWorship => write!(f, "Place of Worship"),
            PlaceType::PointOfInterest => write!(f, "Point of Interest"),
            PlaceType::Political => write!(f, "Political"),
            PlaceType::PostBox => write!(f, "Post Box"),
            PlaceType::PostalCode => write!(f, "Postal Code"),
            PlaceType::PostalCodePrefix => write!(f, "Postal Code Prefix"),
            PlaceType::PostalCodeSuffix => write!(f, "Postal Code Suffix"),
            PlaceType::PostalTown => write!(f, "Postal Town"),
            PlaceType::Premise => write!(f, "Premise"),
            PlaceType::Room => write!(f, "Room"),
            PlaceType::Route => write!(f, "Route"),
            PlaceType::StreetAddress => write!(f, "Street Address"),
            PlaceType::StreetNumber => write!(f, "Street Number"),
            PlaceType::Sublocality => write!(f, "Sublocality"),
            PlaceType::SublocalityLevel1 => write!(f, "Sublocality Level 1"),
            PlaceType::SublocalityLevel2 => write!(f, "Sublocality Level 2"),
            PlaceType::SublocalityLevel3 => write!(f, "Sublocality Level 3"),
            PlaceType::SublocalityLevel4 => write!(f, "Sublocality Level 4"),
            PlaceType::SublocalityLevel5 => write!(f, "Sublocality Level 5"),
            PlaceType::Subpremise => write!(f, "Subpremise"),
            PlaceType::TownSquare => write!(f, "Town Square"),
            PlaceType::Address => write!(f, "Address"),
            PlaceType::Regions => write!(f, "Regions"),
            PlaceType::Cities => write!(f, "Cities"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl PlaceType {
    /// A helper function that converts a `Vec<PlaceType>` (i.e. an array of
    /// PlaceType enum) to a `String` that contains a comma-delimited list of
    /// [place
    /// types](https://developers.google.com/places/web-service/supported_types)
    /// codes.
    pub fn vec_to_csv(place_types: &[PlaceType]) -> String {
        place_types
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(",")
    } // fn
} // impl