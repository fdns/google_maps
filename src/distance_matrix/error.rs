//! Distance Matrix API error types and error messages.

use crate::distance_matrix::response::status::Status;

/// Errors that may be produced by the Google Maps Distance Matrix API client.
#[derive(Debug)]
pub enum Error {
    /// An arrival time may only be specified in Transit travel mode.
    ArrivalTimeIsForTransitOnly(String, String),
    /// Alternatives may not be requested when waypoints are specified.
    EitherAlternativesOrWaypoints(usize),
    /// An departure time may not be specified when an arrival time is
    /// specified.
    EitherDepartureTimeOrArrivalTime(String, String),
    /// Restrictions may not be specified when waypoints are specified.
    EitherRestrictionsOrWaypoints(usize, String),
    /// Waypoints may not be specified in Transit travel mode.
    EitherWaypointsOrTransitMode(usize),
    /// Google Maps Distance Matrix API server generated an error. See the
    /// `Status` enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// avoid/restrictions code. See
    /// `google_maps\src\directions\request\avoid.rs` for more information.
    InvalidAvoidCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// element status code. See
    /// `google_maps\src\distance_matrix\response\element_status.rs` for more
    /// information.
    InvalidElementStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// maneuver type code. See
    /// `google_maps\src\directions\response\maneuver_type.rs` for more
    /// information.
    InvalidManeuverTypeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See `google_maps\src\directions\response\status.rs` for
    /// more information.
    InvalidStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// traffic model code. See
    /// `google_maps\src\directions\request\traffic_model.rs` for more
    /// information.
    InvalidTrafficModelCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// transit mode code. See
    /// `google_maps\src\directions\request\transit_mode.rs` for more
    /// information.
    InvalidTransitModeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// transit routing preference code. See
    /// `google_maps\src\directions\request\transit_route_preference.rs` for
    /// more information.
    InvalidTransitRoutePreferenceCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// travel mode code. See `google_maps\src\directions\travel_mode.rs` for
    /// more information.
    InvalidTravelModeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// unit system code. See
    /// `google_maps\src\directions\request\unit_system.rs` for more
    /// information.
    InvalidUnitSystemCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// vehicle type code. See `google_maps\src\directions\vehicle_type.rs` for
    /// more information.
    InvalidVehicleTypeCode(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Distance Matrix API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "enable-reqwest")]
    Reqwest(reqwest::Error),
    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "enable-reqwest")]
    ReqwestMessage(String),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
    /// Too many waypoints specified.
    TooManyWaypoints(usize),
    /// Transit mode may only be specified in Transit travel mode.
    TransitModeIsForTransitOnly(String, String),
    /// Transit routing preference may only be specified in Transit travel mode.
    TransitRoutePreferenceIsForTransitOnly(String, String),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ArrivalTimeIsForTransitOnly(travel_mode, arrival_time) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_arrival_time() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the arrival time is set to `{arrival_time}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no arrival time."),
            Error::EitherAlternativesOrWaypoints(waypoint_count) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_alternatives() method cannot be set to `true` if with_waypoints() has been set. \
                {waypoint_count} waypoint(s) are set. \
                Try again either with no waypoints or no alternatives."),
            Error::EitherDepartureTimeOrArrivalTime(arrival_time, departure_time) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_departure_time() method cannot be used when with_arrival_time() has been set. \
                The arrival time is set to `{arrival_time}` and the departure time is set to `{departure_time}`. \
                Try again either with no arrival time or no departure time."),
            Error::EitherRestrictionsOrWaypoints(waypoint_count, restrictions) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_restrictions() method cannot be used when with_waypoints() has been set. \
                {waypoint_count} waypoint(s) are set and the restrictions(s) are set to `{restrictions}`. \
                Try again either with no waypoints or no restrictions."),
            Error::EitherWaypointsOrTransitMode(waypoint_count) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_waypoints() method cannot be used when with_travel_mode() is set to `TravelMode::Transit`. \
                {waypoint_count} waypoint(s) are set. \
                Try again either with a different travel mode or no waypoints."),
            Error::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Distance Matrix API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Distance Matrix API service: {error_message}"),
                // If the Google Maps Distance Matrix API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Invalid request. \
                        This may indicate that the query (address, components, or latlng) is missing, an invalid result type, or an invalid location type."),
                    Status::MaxElementsExceeded => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Maximum elements exceeded. \
                        The product of origins and destinations exceeds the per-query limit."),
                    Status::Ok => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Request denied. \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f,
                        "Google Maps Distance Matrix API service: \
                        Unknown error."),
                } // match
            }, // match
            Error::HttpUnsuccessful(status) => write!(f,
                "Google Maps Distance Matrix API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Error::InvalidAvoidCode(avoid_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{avoid_code}` is not a valid restrictions code. \
                Valid codes are `ferries`, `highways`, `indoor`, and `tolls`."),
            Error::InvalidElementStatusCode(element_status_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{element_status_code}` is not a valid geocoder status code. \
                Valid codes are `MAX_ROUTE_LENGTH_EXCEEDED`, `NOT_FOUND`, \
                `OK`, and `ZERO_RESULTS`."),
            Error::InvalidManeuverTypeCode(maneuver_type_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{maneuver_type_code}` is not a valid maneuver type code. \
                Valid codes are `ferry`, `ferry-train`, `fork-left`, \
                `fork-right`, `keep-left`, `keep-right`, `merge`, `ramp-left`, \
                `ramp-right`, `roundabout-left`, `roundabout-right`, \
                `straight`, `turn-left`, `turn-right`, `turn-sharp-left`, \
                `turn-sharp-right`, `turn-slight-left`, `turn-slight-right`, \
                `uturn-left`, and `uturn-right`."),
            Error::InvalidStatusCode(status_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `MAX_ROUTE_LENGTH_EXCEEDED` \
                `MAX_WAYPOINTS_EXCEEDED`, `NOT_FOUND`, `OK`, \
                `OVER_DAILY_LIMIT`, `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, \
                `UNKNOWN_ERROR`, and `ZERO_RESULTS`."),
            Error::InvalidTrafficModelCode(traffic_model_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{traffic_model_code}` is not a valid traffic model code. \
                Valid codes are `best_guess`, `optimistic`, and `pessimistic`."),
            Error::InvalidTransitModeCode(transit_mode_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{transit_mode_code}` is not a valid transit mode code. Valid codes are `bus`,
                `rail`, `subway`, `train`, and `tram`."),
            Error::InvalidTransitRoutePreferenceCode(transit_route_preference_code) =>
                write!(f, "Google Maps Distance Matrix API client: \
                `{transit_route_preference_code}` is not a valid transit route preference code. \
                Valid codes are `fewer_transfers` and `less_walking`."),
            Error::InvalidTravelModeCode(travel_mode_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{travel_mode_code}` is not a valid travel mode code. \
                Valid codes are `bicycling`, `driving`, `transit`, and \
                `walking`."),
            Error::InvalidUnitSystemCode(unit_system_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{unit_system_code}` is not a valid unit system code. \
                Valid codes are `imperial`, and `metric`."),
            Error::InvalidVehicleTypeCode(vehicle_type_code) => write!(f,
                "Google Maps Distance Matrix API client: \
                `{vehicle_type_code}` is not a valid vehicle type code. \
                Valid codes are `BUS`, `CABLE_CAR`, `COMMUTER_TRAIN`,  \
                `FERRY`, `FUNICULAR`, `GONDOLA_LIFT`, `HEAVY_RAIL`, \
                `HIGH_SPEED_TRAIN`, `INTERCITY_BUS`, `LONG_DISTANCE_TRAIN`, \
                `METRO_RAIL`, `MONORAIL`, `OTHER`, `RAIL`, `SHARE_TAXI`, \
                `SUBWAY`, `TRAM`, and `TROLLEYBUS`."),
            Error::QueryNotBuilt => write!(f,
                "Google Maps Distance Matrix API client: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            Error::RequestNotValidated => write!(f,
                "Google Maps Distance Matrix API client: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => write!(f, "Google Maps Distance Matrix API client in the Reqwest library: {error}"),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Error::SerdeJson(error) => write!(f, "Google Maps Distance Matrix API client in the Serde JSON library: {error}"),
            Error::TooManyWaypoints(waypoint_count) => write!(f,
                "Google Maps Distance Matrix API client: \
                The maximum allowed number of waypoints is 25 plus the origin and destination. \
                {} waypoints are set. \
                Try again with {} fewer waypoint(s).",
                waypoint_count,
                waypoint_count - 25),
            Error::TransitModeIsForTransitOnly(travel_mode, transit_modes) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_transit_modes() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the transit mode(s) are set to `{transit_modes}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit modes."),
            Error::TransitRoutePreferenceIsForTransitOnly(travel_mode, transit_route_preference) => write!(f,
                "Google Maps Distance Matrix API client: \
                The with_transit_route_preference() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the transit route preference is set to `{transit_route_preference}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit route preference."),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Distance Matrix
    /// API error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ArrivalTimeIsForTransitOnly(_travel_mode, _arrival_time) => None,
            Error::EitherAlternativesOrWaypoints(_waypoint_count) => None,
            Error::EitherDepartureTimeOrArrivalTime(_arrival_time, _departure_time) => None,
            Error::EitherRestrictionsOrWaypoints(_waypoint_count, _restrictions) => None,
            Error::EitherWaypointsOrTransitMode(_waypoint_count) => None,
            Error::GoogleMapsService(_error, _message) => None,
            Error::HttpUnsuccessful(_status) => None,
            Error::InvalidAvoidCode(_avoid_code) => None,
            Error::InvalidElementStatusCode(_element_status_code) => None,
            Error::InvalidManeuverTypeCode(_maneuver_type_code) => None,
            Error::InvalidStatusCode(_status_code) => None,
            Error::InvalidTrafficModelCode(_traffic_model_code) => None,
            Error::InvalidTransitModeCode(_transit_mode_code) => None,
            Error::InvalidTransitRoutePreferenceCode(_transit_route_preference_code) => None,
            Error::InvalidTravelModeCode(_travel_mode_code) => None,
            Error::InvalidUnitSystemCode(_unit_system_code) => None,
            Error::InvalidVehicleTypeCode(_vehicle_type_code) => None,
            Error::QueryNotBuilt => None,
            Error::RequestNotValidated => None,
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => Some(error),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(_error) => None,
            Error::SerdeJson(error) => Some(error),
            Error::TooManyWaypoints(_waypoint_count) => None,
            Error::TransitModeIsForTransitOnly(_travel_mode, _transit_modes) => None,
            Error::TransitRoutePreferenceIsForTransitOnly(
                _travel_mode,
                _transit_route_preference,
            ) => None,
        } // match
    } // fn
} // impl

#[cfg(feature = "enable-reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Distance Matrix API error type
    /// (`google_maps::distance_matrix::error::Error`) by wrapping it inside.
    /// This function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Distance Matrix API error type
    /// (`google_maps::distance_matrix::error::Error`) by wrapping it inside.
    /// This function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl