use std::{
    char,
    fmt::Debug,
    fs::{read_to_string, File},
    io::{Result, Write},
};

use chrono::{DateTime, Days, Utc};
use rand::{distributions::Alphanumeric, seq::SliceRandom, Rng};

static STATUS_CSV_FILE_NAME: &str = "bcapStatusData.csv";
static QUEUE_CSV_FILE_NAME: &str = "bcapQueueData.csv";
static REQUESTTYPE_CSV_FILE_NAME: &str = "bcapRequestTypeData.csv";
static CUSTOMERREQUEST_CSV_FILE_NAME: &str = "bcapCustRequestData.csv";
static CUSTOMERCASE_CSV_FILE_NAME: &str = "bcapCustCaseData.csv";

fn write_customerrequest_data() {
    let mut file = File::create(CUSTOMERREQUEST_CSV_FILE_NAME).unwrap();
    file.write_all(b"request_id,first_name,middle_name,status_id,request_type_id,last_name,completed_date,completed_by,request_reference_number\n");

    let mut case_file = File::create(CUSTOMERCASE_CSV_FILE_NAME).unwrap();
    case_file.write_all(b"case_id,request_id,assigned_to,locked_to,locked_date_time,complex_case_marker,managed_case_marker,status_id,queue_id,checked_date,checked_by,completed_date,completed_by,form_type\n");

    let last_names_lines: Vec<String> = read_to_string("assets/lastNames.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for (i, element) in read_to_string("assets/firstNames.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let k = last_names_lines.get(i);
        // println!("{}", k.unwrap());

        let request_id = format!("{}", i.to_string());
        let firstN = element.to_string();
        let lastN = k.unwrap();
        let mid = last_names_lines
            .get(rand::thread_rng().gen_range(0..3000))
            .unwrap();
        let status_id = rand::thread_rng().gen_range(1..=3);
        let request_type_id = rand::thread_rng().gen_range(1..=4);

        let completed_date = match status_id {
            2 | 3 => Utc::now()
                .checked_sub_days(Days::new(rand::thread_rng().gen_range(1..=365)))
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            _ => String::new(),
        };

        let completed_by = format!("G001{}", rand::thread_rng().gen_range(0..=100).to_string());
        let request_reference_number: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let write_req_str = format!(
            "{},{},{},{},{},{},{},{},{}\n",
            request_id,
            firstN,
            mid,
            status_id,
            request_type_id,
            lastN,
            completed_date,
            completed_by,
            request_reference_number
        );
        file.write_all(write_req_str.as_bytes());

        if (status_id == 2) {
            let case_id = format!("i{}", i.to_string());
            let assigned_to = format!("G001{}", rand::thread_rng().gen_range(0..=100).to_string());
            let locked_to = format!("G001{}", rand::thread_rng().gen_range(0..=100).to_string());
            let locked_date_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let complex_case_marker = rand::thread_rng().gen_range(0..=1);
            let managed_case_marker = rand::thread_rng().gen_range(0..=1);
            let status_id = rand::thread_rng().gen_range(1..=4);
            let queue_id = if status_id == 1 {
                rand::thread_rng().gen_range(2..=4)
            } else {
                5
            };

            let randDays = Days::new(rand::thread_rng().gen_range(1..=365));
            let randUser = format!("G001{}", rand::thread_rng().gen_range(0..=100).to_string());
            let completed_date = match status_id {
                2 | 3 | 4 => Utc::now()
                    .checked_sub_days(randDays)
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string(),
                _ => String::new(),
            };

            let completed_by = match status_id {
                2 | 3 | 4 => randUser.clone(),
                _ => String::new(),
            };
            let checked_by = randUser;
            let checked_date = Utc::now()
                .checked_sub_days(randDays)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            let possible_form_types = vec!["ACC", "SAR", "DEL", "OBJ"];
            let form_type = possible_form_types[rand::thread_rng().gen_range(0..=3)];
            let write_case_str = format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                case_id,
                request_id,
                assigned_to,
                locked_to,
                locked_date_time,
                complex_case_marker,
                managed_case_marker,
                status_id,
                queue_id,
                checked_date,
                checked_by,
                completed_date,
                completed_by,
                form_type
            );

            case_file.write_all(write_case_str.as_bytes());
        }
    }
}

fn write_status_data() -> std::io::Result<()> {
    let mut file = File::create(STATUS_CSV_FILE_NAME)?;
    file.write_all(b"status_id,status_name\n")?;
    file.write_all(b"1,open\n")?;
    file.write_all(b"2,completed\n")?;
    file.write_all(b"3,rejected\n")?;
    file.write_all(b"4,archived\n")?;

    Ok(())
}

fn write_queue_data() -> std::io::Result<()> {
    let mut file = File::create(QUEUE_CSV_FILE_NAME)?;
    file.write_all(b"queue_id,queue_name\n")?;
    file.write_all(b"1,New-Unassinged\n")?;
    file.write_all(b"2,Work In Progress\n")?;
    file.write_all(b"3,Ready for PDF\n")?;
    file.write_all(b"4,Ready For Dispatch\n")?;
    file.write_all(b"5,Completed\n")?;

    Ok(())
}

fn write_request_types_data() -> std::io::Result<()> {
    let mut file = File::create(REQUESTTYPE_CSV_FILE_NAME)?;
    file.write_all(b"request_type_id,request_type_name\n")?;
    file.write_all(b"1,Product Data Request\n")?;
    file.write_all(b"2,Complaint Data Request\n")?;
    file.write_all(b"3,Personal Info Request\n")?;
    file.write_all(b"4,Prospect Data Request\n")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    write_status_data()?;
    write_queue_data()?;
    write_request_types_data()?;
    write_customerrequest_data();
    // check_time_function();

    let data = None::<u8>;
    Ok(())
}
