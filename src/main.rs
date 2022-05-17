#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

const REASON_NAMES: [&str; 61] = [
    "ранение гр.клетки, живота, спины",
    "ранение головы, шеи (в сознании)",
    "огнестрельное ранение",
    "обширная термическая травма",
    "травматич. ампутация конечностей",
    "множественная, сочетанная травма",
    "задыхается (аллергия)",
    "судороги (высокая температура)",
	"судороги (отравление)",
	"рвота с кровью",
	"роды, кровотечение",
	"роды, отошли воды + кровотечение",
	"болит живот + рвота",
	"без сознания (бронхиальная астма)",
	"без сознания (подавился)",
    "без сознания (угорел)",
	"без сознания (диабет)",
	"без сознания (передозировка нарко)",	
    "без сознания (травма)",
	"без сознания (кардио больной)",
	"без сознания (высокая температура)",
	"без сознания (припадок)",
	"без сознания (отравился)",
	"без сознания (утопление)",
	"без сознания (причина неизвестна)",
	"без сознания (острое кровотечение)",
	"без сознания (анафилактический шок)",
	"без сознания (повесился)",
	"ДТП (1-2, взрослые)",
	"ДТП (1-2, есть дети)",	  	 
	"ДТП (более 2-х, взрослые)", 	  	 
	"ДТП (более 2-х, есть дети)",	  	 
	"наезд на пешехода",
	"пожар (1-2, взрослые)",
	"пожар (1-2, есть дети)",
	"пожар (более 2, взросл.)",
	"пожар (более 2, дети)",
	"взрыв, ЧС (1-2, взрослые)", 	  	 
	"взрыв, ЧС (1-2, есть дети)", 	  	 
	"взрыв, ЧС (более 2, взросл)", 	  	 
	"взрыв, ЧС (более 2, дети)",
	"дежурство при угрозе теракта",
	"падение с высоты (в сознании)",
    "ушиб, перелом конечностей",
    
    "аритмия",
    "парализовало",
    "ДТП (дежурство)",
    "пожар (дежурство)",
    "взрыв, ЧС (дежурство)",

    "передозировка наркотиков",
    "травма глаза",
    "травма уха",
    "повышенное АД",
    "болит живот",
    "роды",
    "роды, отошли воды",
    
    "низкое давление, головокружение",
    "сыпь",
    "без сознания (онко)",
    "вызов на себя фельдшерской бригады",

    "инородное тело носа",
];

const REASON_IMPORTANCE: [&usize; 61] = [
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&1,
	&1,
	&1,
	&1,
	&1,
	&2,
	&2,
	&2,
	&2,
	&2,
	&2,
	&2,
	&3,
	&3,
	&3,
	&3,
	&4,
];

const PATIENT_LOCATION: [&str; 3] = [ 
    "Улица",
    "Дом",
    "Другое",
];

pub fn main(){
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lab 4",
        options,
        Box::new(|_cc| Box::new(LabApp::default())),
    );
}

struct CallInfo {
    id: usize,
    loc_id: usize,
    age: usize,
    conscious: bool,
}

impl Default for CallInfo {
    fn default() -> Self {
        CallInfo { 
            id: 0,
            loc_id: 0,
            age: 0,
            conscious: true,
        }
    }
}

struct LabApp {
    call_info: CallInfo,
    str_age: String,
    importance: usize,
}


impl Default for LabApp {
    fn default() -> Self {
        let info: CallInfo = Default::default();
        Self {
            call_info: info,
            str_age: "18".to_string(),
            importance: 1,
        }
    }
}

fn dump_to_call_info(in_call_info: &mut CallInfo, in_str_age: &mut String) {
    if in_str_age.len() > 0 {
    in_call_info.age = in_str_age.parse::<usize>().unwrap();
    }
}

fn calculate_importance(in_call_info: &mut CallInfo) -> usize
{
    let importance = REASON_IMPORTANCE[in_call_info.id] + 1;
//    if importance == 1 && !(in_call_info.age < 18 || in_call_info.loc_id == 0 || in_call_info.conscious == false){
//        return importance + 1;
//    }
    if !(in_call_info.age < 18 || in_call_info.loc_id == 0 || in_call_info.conscious == false) {
        return importance + 1;
    }
    importance
}

impl eframe::App for LabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Оценка категории срочности вызова");
            ui.horizontal(|ui| {
                ui.label("Возраст: ");
                ui.add(egui::TextEdit::singleline(&mut self.str_age)
                       .hint_text("Поле для ввода возраста")
                       );
            });
            ui.checkbox(&mut self.call_info.conscious, "В сознании");
            ui.label(format!("Степень срочности вызова: {}", REASON_IMPORTANCE[self.call_info.id] + 1));
            egui::ComboBox::from_label("Причина Вызова")
                .show_index(ui, &mut self.call_info.id, REASON_NAMES.len(), |i| REASON_NAMES[i].to_owned());
            egui::ComboBox::from_label("Местоположение пациента")
                .show_index(ui, &mut self.call_info.loc_id, PATIENT_LOCATION.len(), |i| PATIENT_LOCATION[i].to_owned());
            if (ui.add(egui::Button::new("Расчитать"))).clicked() {
                dump_to_call_info(&mut self.call_info, &mut self.str_age);
                self.importance = calculate_importance(&mut self.call_info);
            };
            ui.label(format!("Категория срочности: {}", self.importance));
        });
    }
}
