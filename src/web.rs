
pub fn web_scrape(url: &str, mut data: Vec<Vec<String>>) -> Vec<Vec<String>>{

    for i in 0..data.len() {

        let mut url_all = url.to_string();
        url_all += &data[i][1];

        let response = reqwest::blocking::get(&url_all).unwrap().text().unwrap();

        let resp_data: Vec<&str> = response.split("\n").collect();

        for j in resp_data {

            if j.contains("株価は") && j.contains("円") && j.contains("script type") {

                let resp_data_data: Vec<&str> = j.split("、").collect();
                let mut line_number = 0;

                for k in resp_data_data {
                    if k.contains("円") {
                        line_number += 1;
                        if line_number > 1 { break }

                        let tmp: Vec<&str> = k.split("】").collect();
                        let tmp2: Vec<&str> = tmp[1].split("は").collect();
                        let tmp3: Vec<&str> = tmp2[1].split("円").collect();

                        let mut tmp = tmp3[0].to_string();
                        tmp.retain(|c| c!= ',');
                        if data[i].len() < 5 {
                            data[i].push(tmp);
                        }
                        else {
                            data[i][4] = tmp;
                        }

                    }
                }

            }

        }


    }

    data
}

