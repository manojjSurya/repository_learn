use configuration_parameters::ConfigurationParameters;
use calamine::{Reader,open_workbook_auto, Xlsx,Sheets};
use calamine::open_workbook;
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use macros;
use sdb_io::buf_file_wrtr;

//use sdb_io::

pub fn process_name(
    config_params: &ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
) 
{
#[derive(Debug, Clone)]

    struct FinnoneNPAStruct
    {
        
        FINAL_NPA_STATUS_BANK:String,
        PROVISION_AMOUNT_V1:f64

    };

    let mut finnone_extract_hashmap: HashMap<String, String> = HashMap::new();
    let mut fn_collateral_hashmap: HashMap<String, String> = HashMap::new();
    let mut finnone_npa_hashmap: HashMap<String, FinnoneNPAStruct> = HashMap::new();
    let mut finnone_master_hashmap: HashMap<String, String> = HashMap::new();
    let mut stg_company_details_hashmap: HashMap<String, String> = HashMap::new();
    let mut restructure_merger_hashmap: HashMap<String, String> = HashMap::new();

    let mut non_security_exposure_vec: Vec<&str> = Vec::new();
    let mut finnone_extract_vec:Vec<&str>=Vec::new();
    let mut finnone_npa_vec:Vec<&str>=Vec::new();
    let mut stg_company_details_vec:Vec<&str>=Vec::new();
    let mut i:String;
    
  // println!("the path of I2:{}",config_params.I2_file());
   let finnone_extract_file = File::open(config_params.I2_file()).expect("could not read i2 file");
  //let I2_file = File::open("I2.txt").expect("could not read i2 file");
    let mut finnoneextractreader=BufReader::new(finnone_extract_file);
    let mut finnone_npa_file = File::open(config_params.I3_file()).expect("could not read i3 file"); 
    let mut finnonenpareader=BufReader::new(finnone_npa_file);
    //let mut finnone_master_file = File::open(config_params.I4_file()).expect("could not read i4 file");
    //let mut finnonemasterreader=BufReader::new(finnone_master_file);
    let mut stg_company_details_file= File::open(config_params.I5_file()).expect("could not read i5 file");
    let mut stgdetailscompanyreader=BufReader::new(stg_company_details_file);
    let mut restructure_merged_file = File::open(config_params.I6_file()).expect("could not read i6 file");
    let mut restructuremergedreader=BufReader::new(restructure_merged_file);
    let fn_collateral_file = File::open(config_params.I7_file()).expect("could not read i7 file");
    let mut fncollateralreader=BufReader::new(fn_collateral_file);
         
    
    for (line_num, lines) in finnoneextractreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I2_file(),
                line_num + 1,
                error
            ),
        };
         let finnone_extract_vec = line.split('|').collect::<Vec<&str>>();
        finnone_extract_hashmap.insert(finnone_extract_vec[0].to_string(),finnone_extract_vec[1].to_string());

    }
    for (line_num, lines) in finnonenpareader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I3_file(),
                line_num + 1,
                error
            ),
        };
        finnone_npa_vec = line.split('|').collect::<Vec<&str>>();

        let mut String_Amount:String=finnone_npa_vec[5].to_string();
        let mut PROVISION_AMOUNT_V1:f64=String_Amount.parse().unwrap();
        let mut finnonenpacontents=FinnoneNPAStruct{
            FINAL_NPA_STATUS_BANK:finnone_npa_vec[4].to_string(),
            PROVISION_AMOUNT_V1,
        };
     
       
        finnone_npa_hashmap.insert(finnone_npa_vec[3].to_string(),finnonenpacontents);

    }
   /* for (line_num, lines) in finnonemasterreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I4_file(),
                line_num + 1,
                error
            ),
        };
        let finnone_master_vec = line.split(' ').collect::<Vec<&str>>();
     
        finnone_master_hashmap.insert(finnone_master_vec[0].to_string(), finnone_master_vec[6].to_string());

    }*/
    let mut finnonemasterreader: Xlsx<_> = open_workbook(config_params.I4_file()).expect("unable to read excel");
    if let Some(Ok(reader)) = finnonemasterreader.worksheet_range("Sheet1") {
        for row in reader.rows().skip(1) {
            let Prod_code: String = row[0].to_string();
            let BALM_L2: String = row[6].to_string();
            finnone_master_hashmap.insert(Prod_code,BALM_L2);
        }
    }
    for (line_num, lines) in stgdetailscompanyreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I5_file(),
                line_num + 1,
                error
            ),
        };
        let stg_company_details_vec = line.split('~').collect::<Vec<&str>>();
        //println!("the i5 vec is:{:?}",I5_vec[20]);
        stg_company_details_hashmap.insert(stg_company_details_vec[20].to_string(), stg_company_details_vec[14].to_string());
        //println!("the I5 Hashmap is:{:?}",stg_company_details_hashmap);

    }
    for (line_num, lines) in restructuremergedreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I6_file(),
                line_num + 1,
                error
            ),
        };
        let restructure_merger_vec = line.split("~|").collect::<Vec<&str>>();
        restructure_merger_hashmap.insert(restructure_merger_vec[1].to_string(), "yes".to_string());

    }
    for (line_num, lines) in fncollateralreader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.I7_file(),
                line_num + 1,
                error
            ),
        };
        let fn_collateral_vec = line.split('|').collect::<Vec<&str>>();
        //println!("{:?}",fn_collateral_vec);
      
        fn_collateral_hashmap.insert(fn_collateral_vec[1].to_string(), fn_collateral_vec[6].to_string());

    }
 
    let mut non_security_exposure_file = File::open(config_params.I1_file()).expect("could not read i1 file");
    let mut nonsecurityexposurereader=BufReader::new(non_security_exposure_file);
    let mut output=File::create(config_params.Output_file()).expect("Failed to create a file");
    for line in nonsecurityexposurereader.lines() {
        let non_security_exposure_file_line = line.expect("Failed to read contents of i1 file");
        let mut non_security_exposure_vec:Vec<String> = non_security_exposure_file_line.split("~|").map(|s| s.to_string()).collect();
        let mut Account_Id:String=non_security_exposure_vec[39].to_string();
        let mut Customer_Id:String=non_security_exposure_vec[20].to_string();
        let mut Group_Id:String=non_security_exposure_vec[38].to_string();
        let mut Outstanding_amount:f64=(non_security_exposure_vec[10]).parse().unwrap();
        let mut Outstanding_amount_lcy:f64=(non_security_exposure_vec[10]).parse().unwrap();
        let mut Ccy:String=non_security_exposure_vec[19].to_string();
        let mut Maturity_date:i32=(non_security_exposure_vec[30]).parse().unwrap();
        let mut GL_code=" ";
        let mut Pan_Number=String::new();
        //for(key,value) in &finnone_extract_hashmap{
          if finnone_extract_hashmap.contains_key(&non_security_exposure_vec[39].to_string()) 
          {
             let mut Pan_Number=(finnone_extract_hashmap.get(&non_security_exposure_vec[39].to_string()).unwrap()).to_string();
             println!("{}",Pan_Number);
            
          }
        let mut Customer_Classification_code=String::new();
        if stg_company_details_hashmap.contains_key(&Pan_Number){
            Customer_Classification_code=(stg_company_details_hashmap.get(&Pan_Number).unwrap()).to_string();
            println!("the customer classification code:{}",Customer_Classification_code);
        }
        
        //to print i =NPA
        let mut i=String::new();
        
        if finnone_npa_hashmap.contains_key(&non_security_exposure_vec[39].to_string())
        {
            let NPAStruct=finnone_npa_hashmap.get(&non_security_exposure_vec[39]).unwrap();
            i=NPAStruct.FINAL_NPA_STATUS_BANK.to_string();
        }
        else{
        i="NA".to_string(); 
        }
        let mut provisional_amount:f64=0.0;
        if finnone_npa_hashmap.contains_key(&non_security_exposure_vec[39].to_string())
        {
        let NPAStruct1=finnone_npa_hashmap.get(&non_security_exposure_vec[39]).unwrap();
        provisional_amount=NPAStruct1.PROVISION_AMOUNT_V1;
        }
        else {
            provisional_amount=0.0;
        }
        //proviosnal percentage
        let mut Provisionalpercenatge:f64;
        if (Outstanding_amount_lcy==0.0) || (provisional_amount==0.0)
        {
            Provisionalpercenatge=0.0;

        }
        else{

            Provisionalpercenatge = Outstanding_amount_lcy/provisional_amount;
        }
        //Restructured flag
         let mut restructed_flag=String::new();
         if restructure_merger_hashmap.contains_key(&non_security_exposure_vec[39].to_string())
         {
            restructed_flag="Y".to_string();
         }
         else
         {
            restructed_flag="N".to_string();
         }

        //sanction date
        let mut Sanctiondate=non_security_exposure_vec[29].to_string();
        let mut product_code=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            let mut Product_code=finnone_master_hashmap.get(&non_security_exposure_vec[24]).unwrap().to_string();
            product_code=Product_code.to_string();
        }
        let mut product_description=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            //let Product_description=finnone_master_hashmap.get(&non_security_exposure_vec[24]).unwrap();
            product_description=non_security_exposure_vec[24].to_string();
        }
        else{
            product_description="NA".to_string();
            
        }
        let mut collateral=String::new();
        if finnone_master_hashmap.contains_key(&non_security_exposure_vec[24].to_string()){
            collateral=finnone_master_hashmap.get(&non_security_exposure_vec[24]).unwrap().to_string();
        }
        else {
            collateral="NA".to_string();
        }
        let mut final_output_text = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n",Account_Id,"|",Customer_Id,"|",Group_Id,"|",Outstanding_amount,"|",Outstanding_amount_lcy,"|",Ccy,"|",Maturity_date,"|",GL_code,"|",Pan_Number,"|",i,"|",provisional_amount,"|",Provisionalpercenatge,"|",restructed_flag,"|",Sanctiondate,"|",product_code,"|",product_description,"|",collateral,"|");
         output.write_all(&final_output_text.as_bytes()).expect("Failed to write to a file");
    }

}


