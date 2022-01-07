/*
    rust_ui
    Copyright (C) 2022  Pascal Behmenburg, Jonas Lauschke

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct NoSuitableDeviceError;

impl std::fmt::Display for NoSuitableDeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No vulkan capable devices found.")
    }
}

impl Error for NoSuitableDeviceError {}
