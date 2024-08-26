use std::thread;
use std::time::Duration;

use anyhow::bail;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use winapi::shared::ntdef::HANDLE;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, MODULEENTRY32, TH32CS_SNAPMODULE};
use winapi::um::winnt::PROCESS_ALL_ACCESS;

use crate::services::mugen_stats_reader::MugenStatsReader;

const MUGEN_PROCESS_NAME: &'static str = "game.exe";
const MAX_LOOPS: u16 = 180;
const MAX_READ_ERRORS: u16 = 5;

const AWAIT_TIME_EACH_LOOP: u64 = 1000;
const WIN_OFFSET: usize = 0x001040E8;
const RED_OFFSET: usize = 0x00008728;
const BLUE_OFFSET: usize = 0x0000871C;


struct MugenProcess(u32, HANDLE);

pub struct MugenStatsWinReader;

impl MugenStatsReader for MugenStatsWinReader {
    fn start<'a>(&self, fighter_one: &'a str, fighter_two: &'a str) -> anyhow::Result<&'a str> {
        log::info!("waiting {} seconds to game be fully initialized...", 5);
        thread::sleep(Duration::new(5, 0));

        let mugen_process = Self::attach_process_by_name()?;
        let base_address = Self::get_base_address(mugen_process.0)?;
        let address = Self::read_memory(mugen_process.1, base_address + WIN_OFFSET, 4)?;
        let address_as_int: usize =
            u32::from_le_bytes([address[0], address[1], address[2], address[3]]) as usize;

        let mut n_loops = 0;
        let mut read_errors = 0;
        loop {
            if n_loops > MAX_LOOPS {
                bail!("max loops reached trying to find winner");
            }

            thread::sleep(Duration::from_millis(AWAIT_TIME_EACH_LOOP));

            let (red_value, blue_value) = match (Self::read_memory(mugen_process.1, address_as_int + RED_OFFSET, 4)
                                                 , Self::read_memory(mugen_process.1, address_as_int + BLUE_OFFSET, 4)) {
                (Err(err), _) => {
                    read_errors += 1;
                    if read_errors >= MAX_READ_ERRORS {
                        bail!(err);
                    }
                    continue;
                }
                (_, Err(err)) => {
                    read_errors += 1;
                    if read_errors >= MAX_READ_ERRORS {
                        bail!(err);
                    }
                    continue;
                }
                (Ok(val0), Ok(val1)) => {
                    (u32::from_le_bytes([val0[0], val0[1], val0[2], val0[3]])
                     , u32::from_le_bytes([val1[0], val1[1], val1[2], val1[3]]))
                }
            };


            match (blue_value, red_value) {
                (1, _) => { return Ok(fighter_one); }
                (_, 1) => { return Ok(fighter_two); }
                _ => {
                    n_loops += 1
                }
            }
        }
    }
}

impl MugenStatsWinReader {
    fn attach_process_by_name() -> anyhow::Result<MugenProcess> {
        let mut mugen_pid = 0;
        let s = System::new_all();
        for (_pid, process) in s.processes() {
            if process.name().eq(MUGEN_PROCESS_NAME) {
                log::info!("mugen process has been found with PID: {}", process.pid().as_u32());
                mugen_pid = process.pid().as_u32();
                break;
            }
        }

        if mugen_pid == 0 {
            bail!("mugen process not found...");
        }

        let p_handler = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false as _, mugen_pid) };
        if p_handler.is_null() {
            bail!("error opening mugen process...");
        }

        Ok(MugenProcess(mugen_pid, p_handler))
    }

    fn read_memory(process: HANDLE, address: usize, size: usize) -> anyhow::Result<Vec<u8>> {
        let mut memory: Vec<u8> = Vec::new();
        memory.resize(size, 0);

        unsafe {
            if ReadProcessMemory(
                process,
                address as _,
                memory.as_ptr() as *mut _,
                size,
                0 as _,
            ) == 0
            {
                bail!(GetLastError());
            }
        };

        Ok(memory)
    }

    fn get_base_address(process_id: u32) -> anyhow::Result<usize> {
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process_id) };

        let mut module_entry: MODULEENTRY32 =
            unsafe { std::mem::MaybeUninit::<MODULEENTRY32>::zeroed().assume_init() };
        module_entry.dwSize = std::mem::size_of_val(&module_entry) as u32;

        unsafe {
            if Module32First(snapshot, &mut module_entry) == false as i32 {
                Self::close_handler(snapshot);
                bail!(GetLastError());
            }
        }

        Self::close_handler(snapshot);
        Ok(module_entry.modBaseAddr as usize)
    }

    fn is_process_handler_valid(value: HANDLE) -> bool {
        value != 0 as HANDLE && value != INVALID_HANDLE_VALUE
    }

    fn close_handler(value: HANDLE) -> bool {
        if Self::is_process_handler_valid(value) {
            unsafe { CloseHandle(value) };
            return true;
        }
        return false;
    }
}

