deploy:
	terrctl -loglevel 0 {src,assets}/**

install:
	go get github.com/fastly/terrctl/terrctl/
