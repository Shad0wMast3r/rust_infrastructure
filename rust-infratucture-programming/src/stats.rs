
use sysinfo::*;
pub fn systeminfo() {
let mut sys=System::new_all();
sys.refresh_all(); //constant update
println!("=> disks");
for disk in sys.disks(){
    disk;
}
} 