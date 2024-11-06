use js_sys::Array;
use leaflet::FeatureGroup;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Object;

use crate::create_object_with_properties;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = FeatureGroup)]
    pub type MarkerClusterGroup;

    #[wasm_bindgen(constructor, js_name = "markerClusterGroup", js_namespace = L)]
    pub fn new_with_options(options: &MarkerClusterGroupOptions) -> MarkerClusterGroup;

    #[wasm_bindgen(method, js_name = "addLayers", js_namespace = L)]
    pub fn add_layers(this: &MarkerClusterGroup, layers: &Array) -> MarkerClusterGroup;

    #[wasm_bindgen(method, js_name = "removeLayers", js_namespace = L)]
    pub fn remove_layers(this: &MarkerClusterGroup, layers: &Array) -> MarkerClusterGroup;
}

create_object_with_properties!(
    (MarkerClusterGroupOptions, MarkerClusterGroupOptions),
    (max_cluster_radius, maxClusterRadius, f64),
    (disable_clustering_at_zoom, disableClusteringAtZoom, bool),
    (spiderfy_on_max_zoom, spiderfyOnMaxZoom, bool)
);
