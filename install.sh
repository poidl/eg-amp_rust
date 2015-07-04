LV2DIR=~/.lv2/
BUNDLE=eg-amp_rust.lv2
SONAME=libamp_rust.so
DEST=$LV2DIR$BUNDLE
if [ -d "$DEST" ]; then
    if [ -d "$LV2DIR/$BUNDLE""_backup" ]; then
        rm -r $LV2DIR/$BUNDLE"_backup"
    fi
    mv $DEST $LV2DIR/$BUNDLE"_backup"
fi
cp -r $BUNDLE $LV2DIR
cp target/debug/$SONAME $DEST
