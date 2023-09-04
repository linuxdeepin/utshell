# 生成rust重写指令的so
cargo build --release

# 准备依赖库
sudo mkdir -p /opt/rush/lib/
sudo cp target/release/deps/*.so /opt/rush/lib/

# 配置链接库
sudo cat > /etc/ld.so.conf.d/rush.conf << EOF
/opt/rush/lib
EOF
sudo ldconfig

./configure
make
