// Copyright (C) 2022,  Asymptotic Inc.
//      Author: Taruntej Kanakamalla <taruntej@asymptotic.io>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.
//
// SPDX-License-Identifier: MPL-2.0
#![allow(clippy::non_send_fields_in_send_ty, unused_doc_comments)]

/**
 * plugin-webrtchttp:
 *
 * Since: plugins-rs-0.9.0
 */
use gst::glib;
mod whipsink;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    whipsink::register(plugin)
}

gst::plugin_define!(
    webrtchttp,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MPL",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
