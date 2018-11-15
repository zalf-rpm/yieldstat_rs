extern crate rusqlite;
extern crate time;

//use rusqlite::types::ToSql;
//use time::Timespec;

pub mod yieldstat {

  use std::path::Path;

  type CropId = i32;
  type STTCode = i32;

  pub enum TillageType {
    Plough,
    Conserving,
    NoTillage,
  }

  use rusqlite::{Connection, NO_PARAMS};
  use std::f32;

  pub fn co2_factor(year: i32, scenario: &str, co2_eff_perc_per_ppm_db: f32, cwb: f32) -> f32 {
    let co2: f32 = 380.0; //IrrigationNeed::co2ppm(year, scenario);

    let co2_eff_perc_per_ppm = if cwb > -50.0 {
      co2_eff_perc_per_ppm_db
    } else if cwb < -130.0 {
      co2_eff_perc_per_ppm_db * 1.186
    } else {
      co2_eff_perc_per_ppm_db * (1.0 + 0.186 * ((cwb + 50.0) / 80.0).abs())
    };

    let inc = if year >= 2010 {
      co2_eff_perc_per_ppm * (co2 - 385.0) / 100.0
    } else {
      0.0
    };

    1.0 + inc
  }

  pub struct CropInfo {
    //CropInfo() : id(-1), isCereal(false),
    //proxyCropId(-1), proxyCropAdaptionFactor(1) {}
    id: CropId,
    name: String,
    name_id: String,
    is_cereal: bool,
    proxy_crop_id: CropId,
    proxy_crop_adaption_factor: f64, //bool operator==(const CropInfo& other) const { return id == other.id; }
                                     //bool operator<(const CropInfo& other) const { return id < other.id; }
  }

  pub fn pre_crop_factor(pre_crop_id: CropId, crop_id: CropId) -> f64 {
    let conn = Connection::open(Path::new("./yieldstat.sqlite")).unwrap();

    let mut stmt = conn
      .prepare("SELECT factor FROM yieldstat_precrop_effect where crop_id = ? and precrop_id = ?")
      .unwrap();

    stmt
      .query_row(&[crop_id, pre_crop_id], |row| row.get(0))
      .unwrap()
  }

  pub fn crop_info(crop_id: CropId) -> CropInfo {
    let conn = Connection::open(Path::new("./yieldstat.sqlite")).unwrap();

    let mut stmt = conn
      .prepare("SELECT select id, name_id, name, is_cereal, proxy_crop_id, proxy_adaption_factor FROM yieldstat_crop where crop_id = ? and use_in_landcare = 1")
      .unwrap();

    stmt
      .query_row(&[crop_id], |row| CropInfo {
        id: row.get(0),
        name_id: row.get(1),
        name: row.get(2),
        is_cereal: row.get(3),
        proxy_crop_id: row.get(4),
        proxy_crop_adaption_factor: row.get(5),
      }).unwrap()
  }

  pub fn tillage_factor(
    pre_crop_id: i32,
    crop_id: i32,
    tillage_type: TillageType,
    stt_code: STTCode,
    yearly_precip_sum: f32,
  ) -> f64 {
    let conn = Connection::open(Path::new("./yieldstat.sqlite")).unwrap();

    let mut stmt = conn
      .prepare("SELECT factor FROM yieldstat_precrop_effect where crop_id = ? and precrop_is_cereal = ? and tillage_id = ?")
      .unwrap();

    if stt_code < 131 && yearly_precip_sum < 530.0 {
      match tillage_type {
        TillageType::Plough => return 1.0,
        TillageType::Conserving => return 0.95,
        TillageType::NoTillage => return 0.9,
      }
    }

    stmt
      .query_row(&[crop_id, pre_crop_id], |row| row.get(0))
      .unwrap()
  }

}
