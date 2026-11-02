// minimap mod: `/map` переключает карту в правом верхнем углу через render.set-minimap.
// пути SDK (render::Minimap, render::Corner, имена полей) выведены из имён WIT
// и примера coords-hud, с исходниками SDK не сверялись.
use horizon_mod_sdk::{
    Mod, Registration,
    render::{self, Corner, Minimap},
};

struct WorldMap {
    shown: bool,
}

impl Mod for WorldMap {
    fn init() -> Result<(Self, Registration), String> {
        // on_tick не нужен: карта ставится один раз при каждом переключении
        let hooks = Registration::new().command("map");
        Ok((WorldMap { shown: false }, hooks))
    }

    fn tick(&mut self, _tick: u64) {}

    fn command(&mut self, name: &str, _args: &str) -> Result<Option<String>, String> {
        match name {
            "map" => {
                self.shown = !self.shown;
                let map = if self.shown {
                    Some(Minimap {
                        corner: Corner::TopRight,
                        size: 160.0,
                        blocks_per_pixel: 1.0,
                        rotate: true,
                        round: false,
                    })
                } else {
                    None
                };
                render::set_minimap(map);
                Ok(Some(format!(
                    "Карта {}",
                    if self.shown { "включена" } else { "выключена" }
                )))
            }
            _ => Ok(None),
        }
    }
}

horizon_mod_sdk::export_mod!(WorldMap);
