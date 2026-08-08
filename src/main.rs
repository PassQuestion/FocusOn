use core::task;

use iced::{Application, advanced::graphics::core::{widget, window}, futures::stream::Scan, widget::{Canvas, Column, Grid, Markdown, PickList, Row, Slider, canvas, column, grid, markdown::rule, pick_list, row, slider }};
use iced_draggable_tabs::DraggableTabs;
use camera_project::{Sensor,SENSORS,Subject,SUBJECTS,ARS,ARSES};
//这一部分直接从lib.rs 中的定义去取，把常量切片和结构体的部分外置
#[derive(Debug,Copy)]
enum Message {
    CreateWindow , New , Save , AdjustFocus , AdjustDistance , AdjustObject , AdjustVirtualClips , AdjustGoal , Delete , Choose_Scene , PosibleError
}

fn main() -> iced::Result {
    let mut sensor_chosen:Sensor = SENSORS[0];
    iced::application("FocusOn!",FocusOn::update,FocusOn::view).window(window::Settings{
        maximized: true,
        resizable:true,
        ..window::Settings::default()
    }).run()
}
struct FocusOn {
    focus : f64,
    distance : f64,
    object_width : f64,
    object_height : f64,
    virtual_clips : f64,
    goal : f64,
    global_grid: iced::widget::Grid<Message>,
    tab_count: i32,
}
impl FocusOn {
    pub fn view(&self) -> Grid<'a,Message,Light,>{
        create_all_elements().into(); //听说是Rust的一个特性，在不加分号的情况下，最后一个表达式会隐式作为返回值
 
         
    }
    pub fn create_all_elements(&self) ->iced::Element<Message> {
        let column1 = self::column1_create();
        let column2 = self::column2_create();
        let row1 = self::row1_create();
        let element_grid = grid![ column1 , row1 , column2].columns(1).spacing(10);
        element_grid.into()

        
    }
    pub fn column1_create() ->iced::widget::Column<Message> {
        let scene_tabs = scene_tabs_create();
        let first_column = column(scene_tabs);
        column.into()
    }
    pub fn column2_create() -> iced::widget::Column<Message> {
        let row = row2_create();
        let split_line = rule();
        let illustration = illustration_create();
        let second_column = column![row2,split_line,illustration];
        second_column.into()
    }
    pub fn row1_create() -> iced::widget::Row<Message>{
        let row1 
    }
    pub fn scene_tabs_create() 
        -> iced_draggable_tabs::DraggableTabs<Message> {
        let scenetab = iced_draggable_tabs::DraggableTabs::new(&[T],
            self.active_tabs, 
            |index| Message::Choose_Scene(index),
            |neworder|Message::ChangeSceneOrder(neworder) );
        scenetab.into()
    }
    pub fn illustration_create() -> widget::Element<Message>{
        
    }
    pub fn update(&mut self,message: Message) {
        match message {
            Message::New => {
                Scene_Tabs();
                self.tab_count += 1;
            }
            Message::Save => {

            }
            Message::AdjustFocus => {
                
            }
            Message::AdjustDistance => {

            }
            Message::AdjustVirtualClips => {
                
            }
            Message::AdjustGoal => {
                
            }
            Message::Delete => {
                
                self.tab_count -= 1;
            }
            Message::ChooseScene => {
                
            }
            Message::ChangeSceneOrder => {

            }

        }
    }
}
