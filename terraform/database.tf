resource "azurerm_postgresql_flexible_server" "server" {
  name                = "${local.repository_name}-pgs"
  resource_group_name = azurerm_resource_group.app.name
  location            = azurerm_resource_group.app.location

  administrator_login    = "lerpz"
  administrator_password = "Password1234!"
  sku_name               = "GP_Standard_D2s_v3"

  storage_mb                    = 32768
  version                       = "12"
  auto_grow_enabled             = false
  backup_retention_days         = 7
  public_network_access_enabled = false

  identity {
    type = "UserAssigned"

    identity_ids = [
      azurerm_user_assigned_identity.identity.id
    ]
  }

  lifecycle {
    prevent_destroy = true
  }
}

resource "azurerm_postgresql_database" "primary" {
  name                = "primary"
  resource_group_name = azurerm_resource_group.app.name
  server_name         = azurerm_postgresql_flexible_server.server.name
  charset             = "UTF8"
  collation           = "English_United States.1252"

  lifecycle {
    prevent_destroy = true
  }
}