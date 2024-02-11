pub mod getmyip {
    async fn request_website(url: &String) -> String {
        reqwest::get(url).await.expect("").text().await.expect("")
    }
    pub async fn getmyipv4_1() -> String {
        request_website(&String::from("https://api.ipify.org?format=json")).await
    }

    /*
        pub async fn getmyipv6_1() -> String {
            request_website(&String::from("https://api64.ipify.org?format=json")).await
        }
    */
    pub async fn getmyipv4_2() -> String {
        request_website(&String::from("https://ipv4.jsonip.com/")).await
    }
    /*
        pub async fn getmyipv6_2() -> String {
            // request_website(&String::from("https://ipv6.jsonip.com")).await
            String::from("test ip v6")
        }
    */
}
