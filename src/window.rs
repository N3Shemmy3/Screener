/* window.rs
 *
 * Copyright 2025 Shemmy
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::application::ScreenerApplication;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{
    gio, glib, Accessible, ConstraintTarget, Grid, Native, Root, ShortcutManager, Widget, Window,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/n3shemmy3/Screener/window.ui")]
    pub struct ScreenerWindow {
        // Template widgets
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScreenerWindow {
        const NAME: &'static str = "ScreenerWindow";
        type Type = super::ScreenerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ScreenerWindow {}
    impl WidgetImpl for ScreenerWindow {}
    impl WindowImpl for ScreenerWindow {}
    impl ApplicationWindowImpl for ScreenerWindow {}
    impl AdwApplicationWindowImpl for ScreenerWindow {}
}

glib::wrapper! {
    pub struct ScreenerWindow(ObjectSubclass<imp::ScreenerWindow>)
    @extends Widget, adw::ApplicationWindow, ScreenerApplication,
    @implements gio::ActionGroup, gio::ActionMap, Accessible, gtk::Buildable, ConstraintTarget, Native, Root, ShortcutManager, gtk::ApplicationWindow, Grid, Window;
}

impl ScreenerWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
