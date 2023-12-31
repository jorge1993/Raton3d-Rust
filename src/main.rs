extern crate hidapi_rusb;

use hidapi_rusb::HidError;
use parse_int::parse;
use std::{thread::{sleep_ms, sleep}, time::Duration, net::TcpStream, fmt::format, io::Write};



struct coordinates{
    x:f32,
    y:f32,
    z:f32,
    rx:f32,
    ry:f32,
    rz:f32
}



impl coordinates{
    fn new(x:f32,y:f32,z:f32,rx:f32,ry:f32,rz:f32) -> coordinates{
        coordinates{x,y,z,rx,ry,rz}
    }
    fn get_string(self) -> String{
        format!("[{}, {}, {}, {}, {}, {}]", self.x,self.y,self.z,self.rx,self.ry,self.rz)
    }
    fn get_as_register_string(self) -> String{
        format!("write_output_float_register(0,{})\n
                write_output_float_register(1,{})\n
                write_output_float_register(2,{})\n
                write_output_float_register(3,{})\n
                write_output_float_register(4,{})\n
                write_output_float_register(5,{})\n",
                self.x.to_string(),
                self.y.to_string(),
                self.z.to_string(),
                self.rx.to_string(),
                self.ry.to_string(),
                self.rz.to_string())
    }
}

struct ImplementationDevices {
    manufacture: &'static str,
    product: &'static str,
    vendor_id: u16,
}

const DEVICES_KNOWN: [ImplementationDevices; 1] = [
    ImplementationDevices {
        manufacture: "3Dconnexion",
        product: "SpaceMouse Wireless",
        vendor_id: 9583,
    },
];

fn convert_from_2_bytes_to_i16(bytes0:&u8, bytes1: &u8) -> f32 {
    let mut result: i16 = 0;
    result += (*bytes0 as i16) << 8;
    result += *bytes1 as i16;
    let ret = result as f32;
    ret
}

fn main (){

    let api = hidapi_rusb::HidApi::new().unwrap();
    
    let (mut vid, mut pid) = (0x256F, 0xC62E);
    //let (mut vid, mut pid) = (0x256F, 0xC652);
    // Print out information about all connected devices
    for device in api.device_list() {
        println!("{:04x} : {:04x} : {:#?}", device.vendor_id(), device.product_id(), device.manufacturer_string());
        // if device.manufacturer_string().unwrap() == "3Dconnexion" && device.product_string().unwrap() == "SpaceMouse Wireless" {
        //     let vid =device.vendor_id();
        //     let pid = device.product_id();
        //     print!("{:#?}", vid);
        //     print!("{:#?}", pid);
        // }
        
    }
    
    let devicesss = api.open(vid, pid).unwrap();
    
    //let mut stream = TcpStream::connect("192.168.3.200:30001").unwrap();
    
    // Connect to device using its VID and PID
    while true {
        
        let max_axis: f32 = 350.0;

        // Read data from device
        let mut buf = [0u8; 256];
        let _ = devicesss.read(&mut buf[..]).unwrap();

        // print buffer as array
        // println!("Read: {:?}", &buf[..res]);

        if buf[0] == 1 { // Significa movimiento de raton
            let x:f32 = convert_from_2_bytes_to_i16(&buf[2], &buf[1]) / max_axis;
            let y:f32 = convert_from_2_bytes_to_i16(&buf[4], &buf[3]) / max_axis;
            let z:f32 = convert_from_2_bytes_to_i16(&buf[6], &buf[5]) / max_axis;
            let rx:f32 =convert_from_2_bytes_to_i16(&buf[8], &buf[7]) / max_axis;
            let ry:f32 =convert_from_2_bytes_to_i16(&buf[10], &buf[9]) / max_axis;
            let rz:f32 =convert_from_2_bytes_to_i16(&buf[12], &buf[11]) / max_axis;
            let coordinates = coordinates::new(x,y,z,rx,ry,rz);


            
            print!("x: {:#?} y: {:#?} z: {:#?} rx: {:#?} ry: {:#?} rz: {:#?} \n", coordinates.x, coordinates.y, coordinates.z, coordinates.rx, coordinates.ry, coordinates.rz);
            // println!("rz: {:#?}", coordinates.rz);
            let buffer = [0; 1024];
            //let mut string = format!("sec program():\n{}\nend\n program()\n", coordinates.get_as_register_string());//format!("servoj(get_inverse_kin(pose_trans(get_actual_tcp_pose(), p{}), qnear=get_actual_joint_positions()),0, 0, 1, 0.1, 300)\n" , coordinates.get_string());
            
            //print!("{}", string);
            //stream.write(string.as_bytes()).unwrap();
            //stream.flush().unwrap();

        }
        if buf[0] == 3 { // Significa click en botones
            print!("Boton: {:#?} \n", buf[1]);
        }

    
            
    }
   
}

    
    

