# SeptBlog
SeptBlog is a simple blog platform. It is written in Rust at backend; JavaScript, HTML, CSS, and Bootstrap at the frontend. Septblog was designed to be simple, fast, and secure.

## Deploy
SeptBlog can be deployed to almost every popular operating systems. But here is an simple example to deploy it to a local Linux machine.

### Dependencies
* bash      (To run the installer)
* grep      (To make the installer works)
* rustc     (To compile the source code)
* cargo     (To build the project)
* git       (To clone this repository)

### Clone Repository
```
git clone https://github.com/SeptianTux/septblog.git
cd septblog
```

### Build The Project
```
cd septblog-frontend
cargo build --release
cd ../
cd septblog-backend
cargo build --release
cd ../
```

### Install To The System
Use sudo or equivalen to run the installer script.
```
sudo ./install.sh
```

### Create Database
Create a database to store site's data. SeptBlog uses MySQL database.

### Edit Configuration File
Use your favourite editor with superuser privilage.
Edit ```/etc/septblog/frontend.json``` file and fill all the configuration file need.
Edit ```/etc/septblog/backend.json``` file and fill all the configuration file need.

### Start SeptBlog Frontend Service
```
sudo systemctl start septblog-frontend.service
```

### Start SeptBlog Backend Service
```
sudo systemctl start septblog-backend.service
```

### Install The Site
Go to [http://your-frontend-host:port/install](#) to install the site. Fill all the form provided by the installer. Make sure you finish the installation proccess, the installer will bring you to a page that will tell you if the installation is success if you finish it.

### Login To Admin Panel
After you finish the installation proccess you can access the Admin Panel. The Admin Panel is in [http://your-frontend-host:port/admin](#). Then you can login using account you were registered in the installation process.

## Debug
You can debug SeptBlog service using ```journalctl``` command. At first you need to edit the SeptBlog's systemd service unit.
```
# You can use your favourite editor, but here in the example we use vim.
sudo vim /etc/systemd/system/septblog-backend.service
```

Find line like this :
```
Environment="RUST_LOG=error"
```

Then edit to something like this :
```
Environment="RUST_LOG=debug"
```

And then save the file.

```
# You can use your favourite editor, but here in the example we use vim.
sudo vim /etc/systemd/system/septblog-frontend.service
```

Find line like this :
```
Environment="RUST_LOG=error"
```

Then edit to something like this :
```
Environment="RUST_LOG=debug"
```

And then save the file.

Reinitializes the systemd daemon.
```
sudo systemctl daemon-reload
```

Reload SeptBlog services.
```
sudo systemctl restart septblog-frontend.service
```
```
sudo systemctl restart septblog-backend.service
```

Then you can use journalctl to debug the service.
```
# If you want to debug the frontend.
journalctl -u septblog-frontend.service -f
```
```
# If you want to debug the backend
journalctl -u septblog-backend.service -f
```