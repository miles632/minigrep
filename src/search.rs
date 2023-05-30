/*
 * This is where the multi threaded file search function goes
 */

use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::mpsc;
use std::path::PathBuf;
use std::thread;
