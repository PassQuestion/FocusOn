pub struct Sensor { name: &'static str, width: f64, height: f64}
pub enum Subject { normal { name: &'static str, width: f64, height: f64, description:&'static str}, moon {name: &'static str, width: f64, height: f64, description: &'static str} }
pub struct Ar { option: &'static str, verbose: f64 }
 //这个是我对于原来JS代码的匿名对象的转换，在Rust等语言中已经不太使用对象化编程，九一含名称的结构体类型代替
pub const SENSORS: &[Sensor] = &[
  Sensor { name: "全画幅 Full Frame", width: 36.0, height: 24.0 },
Sensor { name: "APS-C（佳能）",  width: 22.3, height: 14.9 },
Sensor { name: "APS-C（索尼/尼康）", width: 23.5, height: 15.6 },
Sensor { name: "M4/3 微单", width: 17.3, height: 13.0 },
Sensor { name: "1英寸", width: 13.2, height: 8.8  },
Sensor { name: "中画幅 44×33", width: 44.0, height: 33.0 }
];
pub const SUBJECTS: &[Subject] = &[
  Subject::normal{ name: "麻雀",       height: 0.15, width: 0.08, description: "体长约 15 cm" },
  Subject::normal{ name: "斑鸠",       height: 0.30, width: 0.15, description: "体长约 30 cm" },
  Subject::normal{ name: "鸽子",       height: 0.33, width: 0.15, description: "体长约 33 cm" },
  Subject::normal{ name: "喜鹊",       height: 0.45, width: 0.20, description: "体长约 45 cm" },
  Subject::normal{ name: "野鸭（绿头鸭）", height: 0.60, width: 0.25, description: "体长约 60 cm" },
  Subject::normal{ name: "白鹭",       height: 0.60, width: 0.30, description: "身高约 60 cm" },
  Subject::normal{ name: "苍鹭",       height: 0.90, width: 0.45, description: "身高约 90 cm" },
  Subject::normal{ name: "丹顶鹤",     height: 1.40, width: 0.50, description: "身高约 1.4 m" },
  Subject::normal{ name: "天鹅",       height: 1.50, width: 0.55, description: "体长约 1.5 m" },
  Subject::normal{ name: "人像半身",   height: 0.90, width: 0.60, description: "构图高度约 90 cm" },
  Subject::normal{ name: "人像全身",   height: 1.75, width: 0.60, description: "身高约 1.75 m" },
  Subject::normal{ name: "汽车（轿车）", height: 1.45, width: 4.50, description: "车高 1.45 m / 车长 4.5 m" },
  Subject::moon{ name: "月亮（直径3474km）", height: 3474000.0, width: 3474000.0, description: "距地球约 38.44 万 km（距离自动设定）" }
];
pub const ARS: &[Ar] = &[
  Ar { option: "与传感器一致（无裁切）", verbose: 0.0 },
  Ar { option: "4:3",  verbose: 4f64/3f64  },
  Ar { option: "3:2",  verbose: 3f64/2f64  },
  Ar { option: "16:9", verbose: 16f64/9f64 },
  Ar { option: "1:1 方片", verbose: 1.0 }
];
pub fn hyp(a : f64, b : f64) -> f64 {
  return (a.powi(2) + b.powi(2)).powf(0.5)
}
pub fn pct(a : f64) -> String {
  return format!("{:1}",a * 100.0);
}
pub fn fmtFloat(a : f64) -> String {
  let int_a = a as i64;
  let fra_a = a.fract().to_string();
  let result = format!("{:#}",int_a) + fra_a.split_terminator(".").last().unwrap();
  return result;
}
struct CalculationResult {
  
}
pub fn compute(sensor : &Sensor , subject : &Subject , ar : &Ar , focus : f64 , distance : int64 , aim_rate_pctage : f64) -> CalculationResult {
                    /*      统一单位      */
  let distance_micrometer = distance * 1000f64;
  let subject_height,subject_width = subject.height * 1000f64,subject.width * 1000f64;
  let valid = D > f;
  let magnify_rate = (|| 
    if valid {
    focus / (distance_micrometer - focus)
    }else {
        0.0});
  let image_height = (if valid {
    magnify_rate * subject_height
    }else {
        0.0});
  let image_width = (if valid {
    magnify_rate * subject_width
    }else {
        0.0});
  let mut height_cut = subject_height * 1000f64;
  let mut width_cut = subject_width *1000f64;
  if ar.verbose > 0f64 {
    if sensor.width / sensor.height >= ar.verbose {
      height_cut = sensor.height; width_cut = sensor.height * ar.verbose;
    }
    else {
        width_cut = sensor.width; height_cut = sensor.width / ar.verbose;
    } }
    let pctage_for_sensor_height = valid.then(|| image_height / sensor.height);
    let pctage_for_sensor_width = valid.then(|| image_width / sensor.width);
    let pctage_for_image_height = valid.then(|| image_height / sensor.height );
    let pctage_for_image_width = valid.then(|| image_width / sensor.width );
    let target = aim_rate_pctage / 100f64 ; 
    let magnify_needed = (valid && target > 0f64).then(target * sensor.height / subject_height)
 
}
