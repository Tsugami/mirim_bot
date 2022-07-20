use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HHMelody {
  notes: String,
  effect_name: String,
  effect_time: String,
  double_effect_name: String,
  double_effect_time: String,
  max_time: String,
}

fn get_effects(notes: Vec<String>) -> Result<Vec<HHMelody>, Error> {
  let file =  Path::new("hh.json");
  let data = serde_json::from_str<Vec<HHMelody>>(file)?;

  return data.into_iter().filter(|melody| melody.contains())
}