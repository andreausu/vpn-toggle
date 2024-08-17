```shell
rsync -avz vpn-toggle.service root@192.168.1.48:/etc/systemd/system/
sudo systemctl enable vpn-toggle
sudo systemctl start vpn-toggle
```