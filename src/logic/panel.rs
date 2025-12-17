use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, TemplateChild};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/dev/n3shemmy3/Screener/ui/panel.ui")]
pub struct PanelWidget {
    #[template_child]
    pub my_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for PanelWidget {
    const NAME: &'static str = "PanelWidget";
    type Type = PanelWidgetWindow;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PanelWidget {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for PanelWidget {
}

impl WindowImpl for PanelWidget {}

glib::wrapper! {
    pub struct PanelWidgetWindow(ObjectSubclass<PanelWidget>)
        @extends gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PanelWidgetWindow {
    pub fn new(app: &adw::Application) -> Self {
       let window =  glib::Object::builder::<PanelWidgetWindow>()
           .property("application", app)
           .build();
       
       window
    }
}
