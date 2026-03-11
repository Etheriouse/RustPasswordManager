from PySide6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout, QPushButton,
    QListWidget, QListWidgetItem, QLineEdit, QLabel, QFrame, QCheckBox, QMessageBox, QDialog
)
from PySide6.QtGui import QFont
from PySide6.QtCore import Qt
import sys
import os
from pathlib import Path
import platform

master = "Hello"

def get_user_data_dir(app_name: str = None) -> str:
    """
    Renvoie le dossier où stocker les données utilisateur pour une application.
    Si app_name est fourni, crée un sous-dossier pour l'application.
    """
    system = platform.system()
    
    if system == "Windows":
        base_dir = Path(os.getenv("LOCALAPPDATA") or os.getenv("APPDATA") or Path.home())
    elif system == "Darwin":  # macOS
        base_dir = Path.home() / "Library" / "Application Support"
    else:  # Linux et autres
        base_dir = Path.home() / ".local" / "share"
    
    if app_name:
        base_dir = base_dir / app_name

    base_dir.mkdir(parents=True, exist_ok=True)  # crée le dossier s'il n'existe pas
    return str(base_dir)

import vault_rs

class PasswordItem(QWidget):
    def __init__(self, site):
        super().__init__()
        layout = QHBoxLayout()
        layout.setContentsMargins(5, 2, 5, 2)
        layout.setSpacing(10)
        
        self.site = site
        self.is_decrypt = False
    
        self.site_label = QLabel(site)
        self.site_label.setMinimumWidth(150)
        self.site_label.setAlignment(Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignVCenter)
        self.site_label.setFont(QFont("Segoe UI", 11))
        layout.addWidget(self.site_label, 2)
        
        self.username_label = QLabel("●●●●●●")
        self.username_label.setMinimumWidth(150)
        self.username_label.setAlignment(Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignVCenter)
        self.username_label.setFont(QFont("Segoe UI", 11))
        layout.addWidget(self.username_label, 2)
        
        
        self.location_label = QLabel("●●●●●●")
        self.location_label.setMinimumWidth(150)
        self.location_label.setAlignment(Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignVCenter)
        self.location_label.setFont(QFont("Segoe UI", 11))
        layout.addWidget(self.location_label, 2)

        self.password_label = QLabel("●●●●●●")
        self.password_label.setMinimumWidth(150)
        self.password_label.setAlignment(Qt.AlignmentFlag.AlignLeft | Qt.AlignmentFlag.AlignVCenter)
        self.password_label.setFont(QFont("Segoe UI", 11))
        layout.addWidget(self.password_label, 2)

        self.show_btn = QPushButton("👁️")
        self.show_btn.setFixedWidth(40)
        self.show_btn.clicked.connect(self.decrypt_data)
        layout.addWidget(self.show_btn)
        
        self.modify_btn = QPushButton("settings")
        self.modify_btn.setFixedWidth(70)
        self.modify_btn.clicked.connect(self.modify_password)
        layout.addWidget(self.modify_btn)
        
        self.checkbox = QCheckBox()
        self.checkbox.setText("")
        self.checkbox.setStyleSheet("""
                    QCheckBox {
                spacing: 0px;  /* Pas d’espace entre l’indicateur et le texte */
                margin: 0px;
            }
            QCheckBox::indicator {
                width: 20px;
                height: 20px;
                border-radius: 5px;
                border: 2px solid #5c6bc0;
                background-color: transparent;  /* Pas de fond par défaut */
            }
            QCheckBox::indicator:checked {
                background-color: #5c6bc0;
            }
            QCheckBox::indicator:hover {
                border-color: #7986cb;
            }
        """)
        layout.addWidget(self.checkbox)
        
        self.setLayout(layout)
        
    def modify_password(self):
        pass
        #open a window to modify password or modif in champ inside window

    def decrypt_data(self):
        if not self.is_decrypt:
            if vault.verify_master(master):
                data = vault.unlock(master, self.site)
                self.username_label.setText(data.get_username())
                self.password_label.setText(data.get_password())
                self.location_label.setText(data.get_location())    
                self.is_decrypt = True
        

class PasswordManager(QWidget):
    
    def __init__(self):
        super().__init__()
        self.setWindowTitle("💻 Modern Password Manager")
        self.setMinimumSize(1600, 900)
        self.setStyleSheet("""
            QWidget { background-color: #1e1e2f; color: #fff; font-family: 'Segoe UI'; }
            QPushButton { background-color: #5c6bc0; color: #fff; border-radius: 5px; padding: 5px; }
            QPushButton:hover { background-color: #7986cb; }
            QLineEdit { padding: 5px; border-radius: 5px; border: 1px solid #555; background-color: #2e2e44; color: #fff; }
            QListWidget { background-color: #2e2e44; border-radius: 5px; }
            QListWidget::item { background: transparent; }  /* Empêche changement de couleur sur clic */
        """)

        main_layout = QVBoxLayout()

        # Barre d'outils
        toolbar = QHBoxLayout()
        self.site_input = QLineEdit()
        self.site_input.setPlaceholderText("Name")
        self.username_input =  QLineEdit()
        self.username_input.setPlaceholderText("Username")
        self.location_input =  QLineEdit()
        self.location_input.setPlaceholderText("Location")
        self.password_input = QLineEdit()
        self.password_input.setPlaceholderText("Password")
        self.password_input.setEchoMode(QLineEdit.EchoMode.Password)
        
        add_btn = QPushButton("Ajouter")
        add_btn.clicked.connect(self.add_password)
        delete_btn = QPushButton("Supprimer sélection")
        delete_btn.clicked.connect(self.delete_selected)
        toolbar.addWidget(self.site_input)
        toolbar.addWidget(self.username_input)
        toolbar.addWidget(self.location_input)
        toolbar.addWidget(self.password_input)  
        toolbar.addWidget(add_btn)
        toolbar.addWidget(delete_btn)
        main_layout.addLayout(toolbar)

        # Ligne séparatrice
        line = QFrame()
        line.setFrameShape(QFrame.HLine)
        line.setFrameShadow(QFrame.Sunken)
        main_layout.addWidget(line)

        # Liste des mots de passe
        self.password_list = QListWidget()
        self.password_list.setSelectionMode(QListWidget.SelectionMode.NoSelection)  # Empêche la sélection des lignes
        main_layout.addWidget(self.password_list)
        
        for item in vault.get_name_password():
            item_widget = PasswordItem(item)
            list_item = QListWidgetItem()
            list_item.setSizeHint(item_widget.sizeHint())
            self.password_list.addItem(list_item)
            self.password_list.setItemWidget(list_item, item_widget)    
        
        self.setLayout(main_layout)
        
        
    def add_password(self):
        site = self.site_input.text()
        user = self.username_input.text()
        pwd = self.password_input.text()
        locate = self.location_input.text()
        if site and pwd and user and locate:
            # encrypt data and add
            if vault.verify_master(master):
                data = vault_rs.Data(user, pwd, locate)
                vault.lock(master, data, site)
            item_widget = PasswordItem(site)
            list_item = QListWidgetItem()
            list_item.setSizeHint(item_widget.sizeHint())
            self.password_list.addItem(list_item)
            self.password_list.setItemWidget(list_item, item_widget)
            self.site_input.clear()
            self.password_input.clear()
            self.username_input.clear()
            self.location_input.clear()

    def delete_selected(self):
        for i in reversed(range(self.password_list.count())):
            item = self.password_list.item(i)
            widget = self.password_list.itemWidget(item)
            if widget.checkbox.isChecked():
                self.password_list.takeItem(i)
                

class PasswordDialog(QDialog):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("🔒 Mot de passe")
        self.setFixedSize(300, 120)

        layout = QVBoxLayout()

        layout.addWidget(QLabel("Entrez le mot de passe :"))

        self.password_input = QLineEdit()
        self.password_input.setEchoMode(QLineEdit.EchoMode.Password)
        layout.addWidget(self.password_input)

        self.ok_btn = QPushButton("OK")
        self.ok_btn.clicked.connect(self.check_password)
        layout.addWidget(self.ok_btn)

        self.setLayout(layout)
        self.correct = False

    def check_password(self):
        if self.password_input.text() == "pass":  # exemple
            self.correct = True
            self.accept()  # ferme la fenêtre et renvoie QDialog.Accepted
        else:
            QMessageBox.warning(self, "Erreur", "Mot de passe incorrect !")
            self.password_input.clear()

if __name__ == "__main__":
    app = QApplication(sys.argv)
    
    pwd_dialog = PasswordDialog()
    if pwd_dialog.exec() == QDialog.Accepted:
        path_ = get_user_data_dir("dev_rust_passman")
        (vault, salt) = vault_rs.load_vault(path_, "pass")
    
        window = PasswordManager()
        window.show()
        app.exec()
    
        vault_rs.save_vault(path_, vault, "pass", salt)
    else:
        print("Mot de passe non valide, fermeture de l'application.")
    