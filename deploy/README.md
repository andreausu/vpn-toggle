```shell
rsync -avz vpn-toggle.service root@rpi-vpn:/etc/systemd/system/
sudo systemctl enable vpn-toggle
sudo systemctl start vpn-toggle
```