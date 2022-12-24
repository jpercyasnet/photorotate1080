extern crate gtk;
extern crate exif;

use gtk::gdk;
use gtk::glib;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::BufReader;
// use std::io;
use std::fs::File;
use std::process::Command;

use gtk::gdk_pixbuf::{Pixbuf};

use gtk::prelude::*;
use gtk::{
    ProgressBar,
    Label,
    FileChooserDialog,
    FileChooserAction,
    Button,
    ComboBoxText,
    IconView,
    Entry,
    prelude::EntryExt,
    SelectionMode,
    ListStore,
    prelude::TreeModelExt,
    TreeView,
    TreeViewColumn,
    prelude::TreeViewExt,
    CellRendererText,
    Grid,
    Notebook,
    ScrolledWindow,
    prelude::WidgetExt,
};

// These two constants stand for the columns of the listmodel and the listview
const VALUE_COL: i32 = 0;
const IS_DIR_COL: i32 = 1;
// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
button.text-button {
    /* If we don't put it, the yellow background won't be visible */
    border-style: outset;
    border-width: 5px;
    border-color: #888888;
    background-image: none;
}
notebook tab:checked {
    border-style: solid;
    border-width: 5px;
    border-color: blue;
}
#MessTitle {
    font-size: large;
}
#tab1 {
    font-weight: bold;   
    border-style: outset;
    border-width: 5px;
    border-color: #888888;
} 
#tab2 {
    font-weight: bold;   
    border-style: outset;
    border-width: 5px;
    border-color: #888888;
} 
#tab3 {
    font-weight: bold;   
    border-style: outset;
    border-width: 5px;
    border-color: #888888;
} 
/*  progress bar height */
#bar1, progress, trough {
   color: black;
   font-weight: bold;   
   min-height: 15px;
}";

use dump_file::dump_file;

pub fn build_ui(application: &gtk::Application) {

      let provider = gtk::CssProvider::new();
      provider.load_from_data(STYLE.as_bytes());
      gtk::StyleContext::add_provider_for_display(
              &gtk::gdk::Display::default().expect("Could not connect to a display"),
              &provider,
              gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
      );

    let window = gtk::ApplicationWindow::new(application);
    let wtitle = format!("Photo Rotate and Convert to 1080 Rust GTK4 version: {}.{}.{}",gtk::major_version(), gtk::minor_version(), gtk::micro_version());

    window.set_title(Some(&wtitle));
    window.set_size_request(800, 500);

    let messagetitle_label = Label::new(Some("Message: "));
    gtk::prelude::WidgetExt::set_widget_name(&messagetitle_label, "MessTitle");
    let messageval_label = Label::new(Some("Message area"));

    let directory_button = Button::with_label("Directory");
    let directory_combobox = ComboBoxText::new();
    directory_combobox.set_hexpand(true);

    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);

    gtk::prelude::WidgetExt::set_widget_name(&progress_progressbar, "bar1");


// ------- define tabs
    let vnotebook = Notebook::new();      

// -------- first tab
    let v1box = Grid::new();
    v1box.set_column_spacing(5);
    v1box.set_row_spacing(5);

    let listorient_button = Button::with_label("List Orientation");
    let rotateall_button = Button::with_label("Rotate All");
    let tree_view = TreeView::new();
    let column = TreeViewColumn::new();
    let column1 = TreeViewColumn::new();
    let cell = CellRendererText::new();
    let cell1 = CellRendererText::new();
    column.pack_start(&cell, true);
    column1.pack_start(&cell1, true);
    // Associate view's column with model's id column
    column.add_attribute(&cell, "text", 0);
    column1.add_attribute(&cell1, "text", 1);
    column.set_title("File Name");
    column1.set_title("Orientation");
    column.set_sort_column_id(0);
    tree_view.append_column(&column);
    tree_view.append_column(&column1);

    let scroll_window = ScrolledWindow::new();
    scroll_window.set_child(Some(&tree_view));
    scroll_window.set_hexpand(true);
    scroll_window.set_vexpand(true);
    v1box.attach(&listorient_button, 0, 0 , 1, 1);
    v1box.attach(&rotateall_button, 2, 0 , 1, 1);
    v1box.attach(&scroll_window, 0, 1 , 3, 4);


    let tab1_label = Label::new(Some("Rotation Correction"));
    gtk::prelude::WidgetExt::set_widget_name(&tab1_label, "tab1");
    tab1_label.set_width_chars(30);
    vnotebook.append_page(&v1box, Some(&tab1_label));

// -------- second tab
    let v2box = Grid::new();
    v2box.set_column_spacing(5);
    v2box.set_row_spacing(5);

    let listgroup_button = Button::with_label("List");

    gtk::prelude::WidgetExt::set_widget_name(&listgroup_button, "label1");

    let nextgroup_button = Button::with_label("Next Group");
    let firstgroup_button = Button::with_label("First Group");
    let rotateclk_button = Button::with_label("Rotate Clockwise");
    let rotatectclk_button = Button::with_label("Rotate CounterClockwise");
    let rotate180_button = Button::with_label("Rotate 180");
    let from_label = Label::new(Some("From:"));
    let from_entry = Entry::new();
    from_entry.set_text("1");
    let to_label = Label::new(Some("To:"));
    let to_entry = Entry::new();
    to_entry.set_text("16");
    let iconsize_label = Label::new(Some("Icon Size: "));
    let iconsize_entry = Entry::new();
    iconsize_entry.set_text("160");
    let icon_view = IconView::new();
    icon_view.set_pixbuf_column(0); // col 0 of the model
    icon_view.set_text_column(1); // col 1 of the model
    icon_view.set_selection_mode(SelectionMode::Multiple); // note 5
    icon_view.set_columns(0); // note 6
    icon_view.set_item_width(160); // note 7
    let scroll1_window = ScrolledWindow::new();
    scroll1_window.set_child(Some(&icon_view));
    scroll1_window.set_hexpand(true);
    scroll1_window.set_vexpand(true);

    v2box.attach(&listgroup_button, 0, 0 , 1, 1);
    v2box.attach(&nextgroup_button, 1, 0 , 1, 1);
    v2box.attach(&firstgroup_button, 2, 0 , 1, 1);
    v2box.attach(&rotateclk_button, 3, 0 , 1, 1);
    v2box.attach(&rotatectclk_button, 4, 0 , 1, 1);
    v2box.attach(&rotate180_button, 5, 0 , 1, 1);
    v2box.attach(&from_label, 0, 1 , 1, 1);
    v2box.attach(&from_entry, 1, 1 , 1, 1);
    v2box.attach(&to_label, 2, 1 , 1, 1);
    v2box.attach(&to_entry, 3, 1 , 1, 1);
    v2box.attach(&iconsize_label, 4, 1 , 1, 1);
    v2box.attach(&iconsize_entry, 5, 1 , 1, 1);
    v2box.attach(&scroll1_window, 0, 2 , 6, 4);

    let tab2_label = Label::new(Some("Individual Rotation"));
    gtk::prelude::WidgetExt::set_widget_name(&tab2_label, "tab2");
    tab2_label.set_width_chars(30);
    vnotebook.append_page(&v2box, Some(&tab2_label));

// -------- third tab
    let v3box = Grid::new();
    v3box.set_column_spacing(5);
    v3box.set_row_spacing(5);

    let listfrom_button = Button::with_label("List");

    let tree_view2 = TreeView::new();
    let column2 = TreeViewColumn::new();
    let column21 = TreeViewColumn::new();
    let cell2 = CellRendererText::new();
    let cell21 = CellRendererText::new();
    column2.pack_start(&cell2, true);
    column21.pack_start(&cell21, true);
    // Associate view's column with model's id column
    column2.add_attribute(&cell2, "text", 0);
    column21.add_attribute(&cell21, "text", 1);
    column2.set_title("Name");
    column21.set_title("Orientation");
    column2.set_sort_column_id(0);
    tree_view2.append_column(&column2);
    tree_view2.append_column(&column21);

    let scroll_window2 = ScrolledWindow::new();
    scroll_window2.set_child(Some(&tree_view2));
    scroll_window2.set_hexpand(true);
    scroll_window2.set_vexpand(true);

    let dirout_button = Button::with_label("Output Directory");
    let dirout_combobox = ComboBoxText::new();
    dirout_combobox.set_hexpand(true);

    let copy_button = Button::with_label("Copy");

    v3box.attach(&listfrom_button, 0, 0 , 1, 1);
    v3box.attach(&copy_button, 3, 0 , 1, 1);
    v3box.attach(&dirout_button, 0, 1 , 1, 1);
    v3box.attach(&dirout_combobox, 1, 1 , 1, 1);
    v3box.attach(&scroll_window2, 0, 2 , 6, 4);

    let tab3_label = Label::new(Some("Convert to 1080"));
    gtk::prelude::WidgetExt::set_widget_name(&tab3_label, "tab3");
    tab3_label.set_width_chars(30);
    vnotebook.append_page(&v3box, Some(&tab3_label));

    let vbox = Grid::new();
    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);

    vbox.attach(&messagetitle_label, 0, 0 , 1, 1);
    vbox.attach(&messageval_label, 1, 0 , 3, 1);
    vbox.attach(&directory_button, 0, 1 , 1, 1);
    vbox.attach(&directory_combobox, 1, 1 , 3, 1);
    vbox.attach(&vnotebook, 0, 2, 4, 4);
    vbox.attach(&progress_progressbar, 0, 6 , 4, 1);

    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- directory button start -----------------------------------
    directory_button.connect_clicked(glib::clone!(@weak window, @weak directory_combobox, @weak messageval_label => move|_| {
    
        messageval_label.set_text("getting directory");

        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.file() {
                    if let Some(folderpath) = foldername.path() {
                        directory_combobox.prepend_text(&folderpath.display().to_string());
                        directory_combobox.set_active(Some(0));
                        messageval_label.set_text("directory selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING FILE **********</span>");
                }
            }
            if messageval_label.text() == "getting directory" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- directory button end -----------------------------------

//----------------- list orientation button start -----------------------------------
    listorient_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak tree_view => move|_| {

        if let Some(cur_dir) = directory_combobox.active_text() {
            let current_dir = PathBuf::from(&cur_dir);
            let new_model = ListStore::new(&[String::static_type(), String::static_type()]);
            let mut filesize;
            let mut numentry = 0;
            for entry1 in fs::read_dir(&current_dir).unwrap() {
                 let entry = entry1.unwrap();
                 if let Ok(metadata) = entry.metadata() {
                     if let Ok(file_name) = entry.file_name().into_string() {
                         if metadata.is_file() {
                             let file_path = entry.path();
                             if let Err(_e) = dump_file(&file_path) {
                             } else {
                                 let file = File::open(file_path).unwrap();
                                 let reader = exif::Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                                 if let Some(field) = reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                                     if let Some(width) = field.value.get_uint(0) {
                                         filesize = format!("Orientation: {}", width);
                                         if (filesize == "Orientation: 3") |
                                            (filesize == "Orientation: 6") |
                                            (filesize == "Orientation: 8") {
                                             new_model.insert_with_values(None,
                                                   &[(VALUE_COL as u32,&file_name), (IS_DIR_COL as u32, &filesize)]);
                                             numentry = numentry + 1;
                                         }
                                     }
                                 }
                            }
                         }
                     }
                 }
            }
            tree_view.set_model(Some(&new_model));
            if numentry > 0 {
                let msgstr = format!("{} files need rotation correction", numentry);
                messageval_label.set_text(&msgstr);
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List Orientation: directory has no images to rotate **********</span>");
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List Orientation: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- list orientation button end -----------------------------------

//----------------- list  button start -----------------------------------

    listgroup_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak from_entry, @weak to_entry, @weak iconsize_entry, @weak progress_progressbar, @weak icon_view => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        let mut from_int1 = 0;
        let mut to_int1 = 0;
        let mut icon_int1 = 0;
        let mut badsize_int = 1;
        if let Some(cur_dir) = directory_combobox.active_text() {
            if from_entry.text_length() == 0 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM FROM SIZE ENTRY **********</span>");
            } else {
                let input_text = from_entry.text();
                let from_int: i32 = input_text.parse().unwrap_or(-99);
                if from_int > 0 {
                    badsize_int = 0;
                    from_int1 = from_int;
                } else if from_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size is not an integer **********</span>");
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size not positive integer **********</span>");
                }
            }
            if badsize_int == 0 {
                badsize_int = 1;
               if to_entry.text_length() == 0 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM TO SIZE ENTRY **********</span>");
                } else {
                    let inputto_text = to_entry.text();
                    let to_int: i32 = inputto_text.parse().unwrap_or(-99);
                    if to_int > 0 {
                        badsize_int = 0;
                        to_int1 = to_int;
                    } else if to_int == -99 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size is not an integer **********</span>");
                    } else {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size not positive integer **********</span>");
                    }
                }
                if badsize_int == 0 {
                    if to_int1 < from_int1 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size Greater than To Size **********</span>");
                    } else {
                        badsize_int = 1;
                        if iconsize_entry.text_length() != 0 { 
                            let inputic_text = iconsize_entry.text();
                            let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
                            if icon_int > 0 {
                                badsize_int = 0;
                                icon_int1 = icon_int;
                            } else if icon_int == -99 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size is not an integer **********</span>");
                            } else {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not positive integer **********</span>");
                            }
                        } else {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM ICON SIZE ENTRY **********</span>");
                        }
                        if badsize_int == 0 {
                            if (icon_int1 < 50) | (icon_int1 > 255) {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not between 50 and 255 **********</span>");
                            } else {
                                let mut listfull: Vec<String> = Vec::new();
                                let mut listname: Vec<String> = Vec::new();
                                let current_dir = PathBuf::from(&cur_dir);
                                for entry1 in fs::read_dir(&current_dir).unwrap() {
                                     let entry = entry1.unwrap();
                                     if let Ok(metadata) = entry.metadata() {
                                         if let Ok(file_name) = entry.file_name().into_string() {
                                             if metadata.is_file() {
                                                 if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                                    file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                                    file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
                                                     listname.push(file_name.clone());
                                                     let file_path = entry.path().into_os_string().into_string().unwrap();
                                                     listfull.push(file_path.clone());
                                                 }
                                             }
                                         }
                                     }
                                }
                                if listname.len() < from_int1 as usize {
                                    let msgstr = format!("<span color=\"#FF000000\">********* List: From value {} Greater than number of files of {} **********</span>", from_int1, listname.len());
                                    messageval_label.set_markup(&msgstr);
                                } else {
                                    listfull.sort();
                                    listname.sort();
                                    let listnamelen = listname.len();
                                    let new_model = ListStore::new(&[Pixbuf::static_type(), String::static_type()]);
                                    let mut newtoi = to_int1;
                                    if newtoi as usize > listnamelen {
                                        newtoi = listnamelen as i32 ;
                                    }
                                    for indexi in (from_int1 - 1)..newtoi {
                                         let file_pathx = &listfull[indexi as usize];
                                         let pixbufx = Pixbuf::from_file(&file_pathx).unwrap();
                                         let mut pixheight = pixbufx.height();
                                         let mut pixwidth = pixbufx.width();
                                         if pixheight > pixwidth {
                                             pixwidth = icon_int1 * pixwidth / pixheight;
                                             pixheight = icon_int1;
                                         } else {
                                             pixheight = icon_int1 * pixheight / pixwidth;
                                             pixwidth = icon_int1;
                                         }
                                         let pixbuficon: Pixbuf = pixbufx.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                         new_model.insert_with_values(None,
                                            &[(VALUE_COL as u32, &pixbuficon), (IS_DIR_COL as u32,&listname[indexi as usize])]);
                                         let progressfr: f64 = (indexi - from_int1 + 2) as f64 / (newtoi - from_int1 +1) as f64;
                                         progress_progressbar.set_fraction(progressfr);
                                         while glib::MainContext::pending(&glib::MainContext::default()) {
                                      glib::MainContext::iteration(&glib::MainContext::default(),true);
                                         }
                                    }
                                    icon_view.set_model(Some(&new_model));
                                    let msgstr = format!("files from {} to {} displayed of total files {}", from_int1, newtoi, listnamelen);
                                    messageval_label.set_text(&msgstr);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- list  button end -----------------------------------

//----------------- next group  button start -----------------------------------
    nextgroup_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak from_entry, @weak to_entry, @weak iconsize_entry, @weak progress_progressbar, @weak icon_view => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        let mut from_int1 = 0;
        let mut to_int1 = 0;
        let mut icon_int1 = 0;
        let mut badsize_int = 1;
        if let Some(cur_dir) = directory_combobox.active_text() {
            if from_entry.text_length() != 0 { 
                let input_text = from_entry.text(); 
                let from_int: i32 = input_text.parse().unwrap_or(-99);
                if from_int > 0 {
                    badsize_int = 0;
                    from_int1 = from_int;
                } else if from_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size is not an integer **********</span>");
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size not positive integer **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM FROM SIZE ENTRY **********</span>");
            }
            if badsize_int == 0 {
                badsize_int = 1;
                if to_entry.text_length() != 0 { 
                    let inputto_text = to_entry.text(); 
                    let to_int: i32 = inputto_text.parse().unwrap_or(-99);
                    if to_int > 0 {
                        badsize_int = 0;
                        to_int1 = to_int;
                    } else if to_int == -99 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size is not an integer **********</span>");
                    } else {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size not positive integer **********</span>");
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM TO SIZE ENTRY **********</span>");
                }
                if badsize_int == 0 {
                    if to_int1 < from_int1 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size Greater than To Size **********</span>");
                    } else {
                        badsize_int = 1;
                        let oldfrom_int1 = from_int1;
                        from_int1 = to_int1 + 1;
                        to_int1 = to_int1 + to_int1 - oldfrom_int1 + 1;
                        let fromstr = format!("{}", from_int1);
                        let tostr = format!("{}", to_int1);
                        to_entry.set_text(&tostr);
                        from_entry.set_text(&fromstr);
                        if iconsize_entry.text_length() != 0 { 
                            let inputic_text = iconsize_entry.text(); 
                            let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
                            if icon_int > 0 {
                                badsize_int = 0;
                                icon_int1 = icon_int;
                            } else if icon_int == -99 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size is not an integer **********</span>");
                            } else {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not positive integer **********</span>");
                            }
                        } else {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM ICON SIZE ENTRY **********</span>");
                        }
                        if badsize_int == 0 {
                            if (icon_int1 < 50) | (icon_int1 > 255) {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not between 50 and 255 **********</span>");
                            } else {
                                let mut listfull: Vec<String> = Vec::new();
                                let mut listname: Vec<String> = Vec::new();
                                let current_dir = PathBuf::from(&cur_dir);
                                for entry1 in fs::read_dir(&current_dir).unwrap() {
                                     let entry = entry1.unwrap();
                                     if let Ok(metadata) = entry.metadata() {
                                         if let Ok(file_name) = entry.file_name().into_string() {
                                             if metadata.is_file() {
                                                 if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                                    file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                                    file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
                                                     listname.push(file_name.clone());
                                                     let file_path = entry.path().into_os_string().into_string().unwrap();
                                                     listfull.push(file_path.clone());
                                                 }
                                             }
                                         }
                                     }
                                }
                                if listname.len() < from_int1 as usize {
                                    let msgstr = format!("<span color=\"#FF000000\">********* List: From value {} Greater than number of files of {} **********</span>", from_int1, listname.len());
                                    messageval_label.set_markup(&msgstr);
                                } else {
                                    listfull.sort();
                                    listname.sort();
                                    let listnamelen = listname.len();
                                    let new_model = ListStore::new(&[Pixbuf::static_type(), String::static_type()]);
                                    let mut newtoi = to_int1;
                                    if newtoi as usize > listnamelen {
                                        newtoi = listnamelen as i32 ;
                                    }
                                    for indexi in (from_int1 - 1)..newtoi {
                                         let file_pathx = &listfull[indexi as usize];
                                         let pixbufx = Pixbuf::from_file(&file_pathx).unwrap();
                                         let mut pixheight = pixbufx.height();
                                         let mut pixwidth = pixbufx.width();
                                         if pixheight > pixwidth {
                                             pixwidth = icon_int1 * pixwidth / pixheight;
                                             pixheight = icon_int1;
                                         } else {
                                             pixheight = icon_int1 * pixheight / pixwidth;
                                             pixwidth = icon_int1;
                                         }
                                         let pixbuficon: Pixbuf = pixbufx.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                         new_model.insert_with_values(None,
                                            &[(VALUE_COL as u32, &pixbuficon), (IS_DIR_COL as u32, &listname[indexi as usize])]);
                                         let progressfr: f64 = (indexi - from_int1 + 2) as f64 / (newtoi - from_int1 +1) as f64;
                                         progress_progressbar.set_fraction(progressfr);
                                         while glib::MainContext::pending(&glib::MainContext::default()) {
                                             glib::MainContext::iteration(&glib::MainContext::default(),true);
                                         }
                                   }
                                   icon_view.set_model(Some(&new_model));
                                   let msgstr = format!("files from {} to {} displayed of total files {}", from_int1, newtoi, listnamelen);
                                   messageval_label.set_text(&msgstr);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- next group button end -----------------------------------


//----------------- first group  button start -----------------------------------
    firstgroup_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak from_entry, @weak to_entry, @weak iconsize_entry, @weak  progress_progressbar, @weak icon_view => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        let mut from_int1 = 0;
        let mut to_int1 = 0;
        let mut icon_int1 = 0;
        let mut badsize_int = 1;
        if let Some(cur_dir) = directory_combobox.active_text() {
            if from_entry.text_length() != 0 { 
                let input_text = from_entry.text(); 
                let from_int: i32 = input_text.parse().unwrap_or(-99);
                if from_int > 0 {
                    badsize_int = 0;
                    from_int1 = from_int;
                } else if from_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size is not an integer **********</span>");
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size not positive integer **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM FROM SIZE ENTRY **********</span>");
            }
            if badsize_int == 0 {
                badsize_int = 1;
                if to_entry.text_length() != 0 { 
                    let inputto_text = to_entry.text(); 
                    let to_int: i32 = inputto_text.parse().unwrap_or(-99);
                    if to_int > 0 {
                        badsize_int = 0;
                        to_int1 = to_int;
                    } else if to_int == -99 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size is not an integer **********</span>");
                    } else {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: To Size not positive integer **********</span>");
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM TO SIZE ENTRY **********</span>");
                }
                if badsize_int == 0 {
                    if to_int1 < from_int1 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* List: From Size Greater than To Size **********</span>");
                    } else {
                        badsize_int = 1;
                        to_int1 = to_int1 - from_int1 + 1;
                        from_int1 = 1;
                        let fromstr = format!("{}", from_int1);
                        let tostr = format!("{}", to_int1);
                        to_entry.set_text(&tostr);
                        from_entry.set_text(&fromstr);
                        if iconsize_entry.text_length() != 0 { 
                            let inputic_text = iconsize_entry.text(); 
                            let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
                            if icon_int > 0 {
                                badsize_int = 0;
                                icon_int1 = icon_int;
                            } else if icon_int == -99 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size is not an integer **********</span>");
                            } else {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not positive integer **********</span>");
                            }
                        } else {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING TEXT FROM ICON SIZE ENTRY **********</span>");
                        }
                        if badsize_int == 0 {
                            if (icon_int1 < 50) | (icon_int1 > 255) {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not between 50 and 255 **********</span>");
                            } else {
                                let mut listfull: Vec<String> = Vec::new();
                                let mut listname: Vec<String> = Vec::new();
                                let current_dir = PathBuf::from(&cur_dir);
                                for entry1 in fs::read_dir(&current_dir).unwrap() {
                                     let entry = entry1.unwrap();
                                     if let Ok(metadata) = entry.metadata() {
                                         if let Ok(file_name) = entry.file_name().into_string() {
                                             if metadata.is_file() {
                                                 if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                                    file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                                    file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
                                                     listname.push(file_name.clone());
                                                     let file_path = entry.path().into_os_string().into_string().unwrap();
                                                     listfull.push(file_path.clone());
                                                 }
                                             }
                                         }
                                     }
                                }
                                if listname.len() < from_int1 as usize {
                                    let msgstr = format!("<span color=\"#FF000000\">********* List: From value {} Greater than number of files of {} **********</span>", from_int1, listname.len());
                                    messageval_label.set_markup(&msgstr);
                                } else {
                                    listfull.sort();
                                    listname.sort();
                                    let listnamelen = listname.len();
                                    let new_model = ListStore::new(&[Pixbuf::static_type(), String::static_type()]);
                                    let mut newtoi = to_int1;
                                    if newtoi as usize > listnamelen {
                                        newtoi = listnamelen as i32 ;
                                    }
                                    for indexi in (from_int1 - 1)..newtoi {
                                         let file_pathx = &listfull[indexi as usize];
                                         let pixbufx = Pixbuf::from_file(&file_pathx).unwrap();
                                         let mut pixheight = pixbufx.height();
                                         let mut pixwidth = pixbufx.width();
                                         if pixheight > pixwidth {
                                             pixwidth = icon_int1 * pixwidth / pixheight;
                                             pixheight = icon_int1;
                                         } else {
                                             pixheight = icon_int1 * pixheight / pixwidth;
                                             pixwidth = icon_int1;
                                         }
                                         let pixbuficon: Pixbuf = pixbufx.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                         new_model.insert_with_values(None,
                                            &[(VALUE_COL as u32, &pixbuficon), (IS_DIR_COL as u32, &listname[indexi as usize])]);
                                         let progressfr: f64 = (indexi - from_int1 + 2) as f64 / (newtoi - from_int1 +1) as f64;
                                         progress_progressbar.set_fraction(progressfr);
                                         while glib::MainContext::pending(&glib::MainContext::default()) {
                                             glib::MainContext::iteration(&glib::MainContext::default(),true);
                                         }
                                    }
                                    icon_view.set_model(Some(&new_model));
                                    let msgstr = format!("files from {} to {} displayed of total files {}", from_int1, newtoi, listnamelen);
                                    messageval_label.set_text(&msgstr);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- first group button end -----------------------------------

//----------------- rotate all button start -----------------------------------
    rotateall_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak tree_view, @weak progress_progressbar => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }

        let treemodel = tree_view.model();
        if treemodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate All: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(cur_dir) = directory_combobox.active_text() {
                let mut numrot = 0;
                let treemodeluw = treemodel.unwrap();
                let mut valid = true;
                let validval = treemodeluw.iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = treemodeluw.iter_n_children(None);
                let mut msgvar = format!(" ");
                let mut numprocess = 0;
                while valid {
                      let treeval = treemodeluw.get_value(&validval,1).get::<String>();
                      let filenameval = treemodeluw.get_value(&validval,0).get::<String>();
                      valid = treemodeluw.iter_next(&validval);
                      let strval = format!("{:?}", treeval);
                      let locind = strval.find("Orientation");
                      if locind != None {
                          let start = locind.unwrap();
                          let start = start + 13;
                          let end = start + 1;
                          let getorient = strval.get(start..end);
                          let orient_int: i32 = getorient.unwrap().parse().unwrap_or(-99);
                          if orient_int > 0 {
                              if (orient_int == 3) | 
                                 (orient_int == 6) |
                                 (orient_int == 8) {
                                  numrot = numrot + 1;
//                                  let filenamestr = format!("{:?}", filenameval);
//                                  let fileln = filenamestr.len();
//                                  let fileend = fileln - 3;
//                                  let filestart = 9;
                                  let filenamex = filenameval.unwrap().to_string();
                                  let s1_param = format!("{}/{}", cur_dir, filenamex);
//         				          println!("s1_param: {}", s1_param);

                                  if valid & (numprocess < 4) {
                                      Command::new("/home/jp/gimp.sh")
                                             .arg(&s1_param)
                                             .spawn()
                                             .expect("failed to execute process");
                                      numprocess = numprocess + 1;
                                  } else {
                                      let _output = Command::new("/home/jp/gimp.sh")
                                                               .arg(&s1_param)
                                                               .output()
                                                               .expect("failed to execute process");
                                      numprocess = 0;
                                  }
                              }
                          } else if orient_int == -99 {
                              msgvar = format!(" {} File {:?} orientation value of {:?} is not an integer", msgvar, filenameval, getorient);
                          } else {
                              msgvar = format!(" {} File {:?} orientation value of {} is not positive", msgvar, filenameval, orient_int);
                          }
                      }
                      numrow = numrow + 1;
                      let progressfr: f64 = numrow as f64 / numchildren as f64;
                      progress_progressbar.set_fraction(progressfr);
                      while glib::MainContext::pending(&glib::MainContext::default()) {
                             glib::MainContext::iteration(&glib::MainContext::default(),true);
                      }
                }
                let msgstr = format!("Number of files rotated: {} {}", numrot, msgvar);
                messageval_label.set_text(&msgstr);
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
            }
        }
    }));
//----------------- rotate all button end -----------------------------------

//----------------- rotate clockwise button start -----------------------------------
    rotateclk_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak icon_view => move|_| {

        let iconmodel = icon_view.model();
        if iconmodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(cur_dir) = directory_combobox.active_text() {
                let icon_selectpath = icon_view.selected_items();
                if icon_selectpath.len() > 0 {
                    let iconmodeluw = iconmodel.unwrap();
                    let pathlen = icon_selectpath.len();
                    let mut numrot = 0;
                    let mut numprocess = 0;
                    for path in &icon_selectpath {
                         numrot = numrot + 1;
                         let iconiter = iconmodeluw.iter(&path).unwrap();
                         let filenameval = iconmodeluw.get_value(&iconiter,1).get::<String>();
//                         let filenamestr = format!("{:?}", filenameval);
//                         let fileln = filenamestr.len();
//                         let fileend = fileln - 3;
//                         let filestart = 9;
                         let filenamex = filenameval.unwrap().to_string();
                         let s1_param = format!("{}/{}", cur_dir, filenamex);
                         if (numrot < pathlen) & (numprocess < 4) {
                             Command::new("/home/jp/gimprotck.sh")
                                          .arg(&s1_param)
                                          .spawn()
                                          .expect("failed to execute process");
                             numprocess = numprocess + 1;
                         } else {
                             let _output = Command::new("/home/jp/gimprotck.sh")
                                                       .arg(&s1_param)
                                                       .output()
                                                       .expect("failed to execute process");
                             numprocess = 0;
                         }
                    }
                    let msgstr = format!("Number of files rotated clockwise: {}", numrot);
                    messageval_label.set_text(&msgstr);
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: NOTHING SELECTED **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: error getting current directory **********</span>");
            }
        }
    }));
//----------------- rotate clockwise button end -----------------------------------

//----------------- rotate counter clockwise button start -----------------------------------
    rotatectclk_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak icon_view => move|_| {

        let iconmodel = icon_view.model();
        if iconmodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(cur_dir) = directory_combobox.active_text() {
                let icon_selectpath = icon_view.selected_items();
                if icon_selectpath.len() > 0 {
                    let iconmodeluw = iconmodel.unwrap();
                    let pathlen = icon_selectpath.len();
                    let mut numrot = 0;
                    let mut numprocess = 0;
                    for path in &icon_selectpath {
                         numrot = numrot + 1;
                         let iconiter = iconmodeluw.iter(&path).unwrap();
                         let filenameval = iconmodeluw.get_value(&iconiter,1).get::<String>();
//                         let filenamestr = format!("{:?}", filenameval);
//                         let fileln = filenamestr.len();
//                         let fileend = fileln - 3;
//                         let filestart = 9;
                         let filenamex = filenameval.unwrap().to_string();
                         let s1_param = format!("{}/{}", cur_dir, filenamex);
                         if (numrot < pathlen) & (numprocess < 4) {
                             Command::new("/home/jp/gimprotcck.sh")
                                          .arg(&s1_param)
                                          .spawn()
                                          .expect("failed to execute process");
                             numprocess = numprocess + 1;
                         } else {
                             let _output = Command::new("/home/jp/gimprotcck.sh")
                                                       .arg(&s1_param)
                                                       .output()
                                                       .expect("failed to execute process");
                             numprocess = 0;
                         }
                    }
                    let msgstr = format!("Number of files rotated counter clockwise: {}", numrot);
                    messageval_label.set_text(&msgstr);
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: NOTHING SELECTED **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: error getting current directory **********</span>");
            }
        }
    }));
//----------------- rotate counter clockwise button end -----------------------------------

//----------------- rotate 180 button start -----------------------------------
    rotate180_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak icon_view => move|_| {

        let iconmodel = icon_view.model();
        if iconmodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(cur_dir) = directory_combobox.active_text() {
                let icon_selectpath = icon_view.selected_items();
                if icon_selectpath.len() > 0 {
                    let iconmodeluw = iconmodel.unwrap();
                    let pathlen = icon_selectpath.len();
                    let mut numrot = 0;
                    let mut numprocess = 0;
                    for path in &icon_selectpath {
                         numrot = numrot + 1;
                         let iconiter = iconmodeluw.iter(&path).unwrap();
                         let filenameval = iconmodeluw.get_value(&iconiter,1).get::<String>();
//                         let filenamestr = format!("{:?}", filenameval);
//                         let fileln = filenamestr.len();
//                         let fileend = fileln - 3;
//                         let filestart = 9;
                         let filenamex = filenameval.unwrap().to_string();
                         let s1_param = format!("{}/{}", cur_dir, filenamex);
                         if (numrot < pathlen) & (numprocess < 4) {
                             Command::new("/home/jp/gimprot180.sh")
                                          .arg(&s1_param)
                                          .spawn()
                                          .expect("failed to execute process");
                             numprocess = numprocess + 1;
                         } else {
                             let _output = Command::new("/home/jp/gimprot180.sh")
                                                       .arg(&s1_param)
                                                       .output()
                                                       .expect("failed to execute process");
                             numprocess = 0;
                         }
                    }
                    let msgstr = format!("Number of files rotated 180: {}", numrot);
                    messageval_label.set_text(&msgstr);
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: NOTHING SELECTED **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate Clockwise: error getting current directory **********</span>");
            }
        }
    }));
//----------------- rotate 180 button end -----------------------------------

//----------------- list for convert button start -----------------------------------
    listfrom_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak tree_view2 => move|_| {

        if let Some(cur_dir) = directory_combobox.active_text() {
            let current_dir = PathBuf::from(&cur_dir);
            let new_model = ListStore::new(&[String::static_type(), String::static_type()]);
            let mut filesize;
            let mut numentry = 0;
            for entry1 in fs::read_dir(&current_dir).unwrap() {
                 let entry = entry1.unwrap();
                 if let Ok(metadata) = entry.metadata() {
                     if let Ok(file_name) = entry.file_name().into_string() {
                         if metadata.is_file() {
                             let file_path = entry.path();
                             if let Err(e) = dump_file(&file_path) {
                                 filesize = format!("Meta error : {}", e);
                             } else {
                                 let file = File::open(file_path).unwrap();
                                 let reader = exif::Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                                 if let Some(field) = reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                                     if let Some(width) = field.value.get_uint(0) {
                                         filesize = format!("Orientation: {}", width);
                                     } else {
                                         filesize = format!("no Value: {}", file_name);
                                     }
                                 } else {
                                     filesize = format!("error getting Value: {}", file_name);
                                 }
                            }
                            new_model.insert_with_values(None,
                                       &[(VALUE_COL as u32, &file_name), (IS_DIR_COL as u32, &filesize)]);
                            numentry = numentry + 1;
                         }
                     }
                 }
            }
            tree_view2.set_model(Some(&new_model));
            if numentry > 0 {
                let msgstr = format!("{} files in directory", numentry);
                messageval_label.set_text(&msgstr);
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List for convert: directory has no images **********</span>");
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List for convert: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- list for convert button end -----------------------------------

//----------------- directory output button start -----------------------------------
    dirout_button.connect_clicked(glib::clone!(@weak window, @weak dirout_combobox, @weak messageval_label => move|_| {

        messageval_label.set_text("getting directory output");

        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.file() {
                    if let Some(folderpath) = foldername.path() {
                        dirout_combobox.prepend_text(&folderpath.display().to_string());
                        dirout_combobox.set_active(Some(0));
                        messageval_label.set_text("directory output selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Directory output: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory output: ERROR GETTING FILE **********</span>");
                }
            }
            if messageval_label.text() == "getting directory output" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory output: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- directory button end -----------------------------------


//----------------- copy convert button start -----------------------------------
    copy_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak dirout_combobox, @weak messageval_label, @weak tree_view2, @weak progress_progressbar => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }

        let mut msgstr = format!(" ");
        let mut bolok = true;

        let treemodel = tree_view2.model();
        if treemodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Copy convert: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(from_dir) = directory_combobox.active_text() {
                if let Some(to_dir) = dirout_combobox.active_text() {
                    let topath_dir = PathBuf::from(&to_dir);
                    let mut numfiles = 0;
                    for entry1 in fs::read_dir(&topath_dir).unwrap() {
                          let entry = entry1.unwrap();
                          if let Ok(metadata) = entry.metadata() {
                              if metadata.is_file() {
                                   numfiles = numfiles + 1;
                              }
                          }
                    }
                    if numfiles > 0 {
                         messageval_label.set_markup("<span color=\"#FF000000\">********* Copy convert: ERROR Output directory is not empty ******</span>");
                    } else {
                         let mut numrot = 0;
                         let treemodeluw = treemodel.unwrap();
                         let mut valid = true;
                         let validval = treemodeluw.iter_first().unwrap();
                         let mut numrow = 0;
                         let numchildren = treemodeluw.iter_n_children(None);
                         let mut numprocess = 0;
                         while valid {
                             let filenameval = treemodeluw.get_value(&validval,0).get::<String>();
                             valid = treemodeluw.iter_next(&validval);
                             numrot = numrot + 1;
//                             let filenamestr = format!("{:?}", filenameval);
//                             let fileln = filenamestr.len();
//                             let fileend = fileln - 3;
//                             let filestart = 9;
                             let filenamex = filenameval.unwrap().to_string();
                             let s1_param = format!("{}/{}", from_dir, filenamex);
                             let s2_param = format!("{}/{}", to_dir, filenamex);
                             if !Path::new(&s1_param).exists() {
                                 msgstr = format!("<span color=\"#FF000000\">********* Copy convert: ERROR {} does not exist **********</span>", s1_param);
                                 bolok = false;
                                 break;
                             }
                             if Path::new(&s2_param).exists() {
                                 msgstr = format!("<span color=\"#FF000000\">********* Copy convert: ERROR {} already exists **********</span>", s2_param);
                                 bolok = false;
                                 break;
                             }

                             if valid & (numprocess < 4) {
                                      Command::new("convert")
                                             .arg(&s1_param)
                                             .arg("-resize")
                                             .arg("1920x1080")
                                             .arg("-background")
                                             .arg("black")
                                             .arg("-gravity")
                                             .arg("center")
                                             .arg("-extent")
                                             .arg("1920x1080")
                                             .arg(&s2_param)
                                             .spawn()
                                             .expect("failed to execute process");
                                      numprocess = numprocess + 1;
                             } else {
                                      let _output = Command::new("convert")
                                             .arg(&s1_param)
                                              .arg("-resize")
                                             .arg("1920x1080")
                                             .arg("-background")
                                             .arg("black")
                                             .arg("-gravity")
                                             .arg("center")
                                             .arg("-extent")
                                             .arg("1920x1080")
                                             .arg(&s2_param)
                                             .output()
                                             .expect("failed to execute process");
                                      numprocess = 0;
                             } 
                             numrow = numrow + 1;
                             let progressfr: f64 = numrow as f64 / numchildren as f64;
                             progress_progressbar.set_fraction(progressfr);
                             while glib::MainContext::pending(&glib::MainContext::default()) {
                                    glib::MainContext::iteration(&glib::MainContext::default(),true);
                             }
                         }
                         if bolok {
                             msgstr = format!("Number of files copied: {}", numrot);
                             messageval_label.set_text(&msgstr);
                         } else {
                             messageval_label.set_markup(&msgstr);
                         }
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING output DIRECTORY IN COMBOBOX **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
            }
        }
    }));
//----------------- copy convert button end -----------------------------------

//------------------- connects end
}
