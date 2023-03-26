# PV191 Smart Home Server

[![CI](https://github.com/KristianMika/pv191-smart-home-server/actions/workflows/ci.yaml/badge.svg)](https://github.com/KristianMika/pv191-smart-home-server/actions/workflows/ci.yaml)

## Temperature and Humidity

[DHT22](https://pdf1.alldatasheet.com/datasheet-pdf/view/1132459/ETC2/DHT22.html) is a capacitive-type humidity and temperature sensor. It measures relative humidity on the range 0-100%RH with precision of +-5% and temperature on range -40°C - 80°C with precision < +-0.5°C.

## Volatile Organic Compounds (VOC)

The setup utilizes Adafruit [SGP40 Air Quality Sensor](https://docs.rs-online.com/1956/A700000007055193.pdf). The sensor uses the aforementioned DHT22 sensor to provide data for computation of compensated VOC index. The index is computed using [Sensirion VOC algorithm](https://github.com/Sensirion/gas-index-algorithm). The algorithm outputs a natural number in range [0, 500] signalising indoor air quality. Values below 100 indicate a typical indoor gas composition, while values above 100 indicate detoriated air quality.

![VOC scale](./.github/images/voc_scale.jpeg)
