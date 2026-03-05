build:
	mkdir -p build/
	cp ui/main.py build/ui.py
	cp target/wheels/password_manager-0.1.0-cp312-cp312-manylinux_2_34_x86_64.whl build/lib_vault_rs.whl
	pyinstaller --onefile ui.py

clean:
	rm -rf build/