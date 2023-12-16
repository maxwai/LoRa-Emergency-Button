# LoRa Emergency Button

This is the code that I used to build an emergency button system for my Grandmother.

Commercial systems in Germany mostly work in a button + basis station configuration. This isn't bad, since I do the same here, but the communication between both devices is always via 2.4 Ghz or Bluetooth. This means that the range is fine for a small house/apartment but wouldn't work for a bigger house, especially when in the garden.

# Hardware Setup

## Button

For the button, I chose a generic LoRa (433 Mhz) emergency button. The button doesn't really matter as long as it transmit some kind of message with LoRa when pressed.

## Basis station

For the basis station I chose a Raspberry Pi 4B and a [SX1268 433M LoRa HAT](https://www.waveshare.com/wiki/SX1262_868M_LoRa_HAT) from Waveshare.

# Software

The Basis station has only a few components:

* A LoRa hat listener to react to a button press.
* A Heartbeat thread, that periodically calls a push url from [Uptime Kuma](https://github.com/louislam/uptime-kuma) so that the status of the basis station can be watched independently.
* An SMS Service that sends SMS to the configured numbers in case the button is pressed.