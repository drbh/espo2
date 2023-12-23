# espo2

What do you get when you combine an ESP32, a SCD30 CO₂ sensor and Rust?

You get `espo2`

This is a minimal embedded program that reads the CO₂ concentration from the SCD30 sensor and prints it to the serial console.

### 📸 Build and flash

Ensure that you're ESP32 is connected to your computer and that you have the toolchain installed (see below). After that, you can build and flash the program to your board.

```bash
cargo espflash flash --release --monitor
# you can skip flashing and just monitor with `cargo espflash monitor`
```

### 🐣 First time setup

If you're a embedded newbie like me, you might need to install some tools first.

```bash
# install the toolchain
cargo install espup
espup install
. ~/export-esp.sh

# install the flasher
cargo install espflash
```

### 🎮 Wiring

You'll only need 4 wires to connect the SCD30 sensor to the ESP32 over I²C.

The SCD30 sensor uses I²C to communicate with the ESP32. The wiring is as follows:

| SCD30 | ESP32 |
| ----- | ----- |
| VDD   | 3.3V  |
| GND   | GND   |
| SCL   | 22    |
| SDA   | 21    |

### 🌈 Notes

The total cost of this project was ~
| Part | Price with shipping |
| ---- | ------------------- |
| ESP32 | $5.97 |
| SCD30 | $22.03 |
| Total | $28.00 |

The accuracy of the SCD30 sensor is ±(30 ppm + 3% of reading) for CO₂ concentrations between 400 and 10000 ppm. The SCD30 is a NDIR sensor which are generally more accurate than other types of CO₂ sensors and this level of accuracy is well within my casual needs.

- [Datasheet](https://www.sensirion.com/media/documents/4EAF6AF8/61652C3C/Sensirion_CO2_Sensors_SCD30_Datasheet.pdf)
- [OEM Product Page](https://www.sensirion.com/products/catalog/SCD30/)
