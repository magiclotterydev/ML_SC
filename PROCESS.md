PDA A = `GLOBAL_PDA_AUTHORITY`
PDA B = `COLLECTION_STTINGS` - (unique pour chaque collection + store the collection settings)
PDA C = `LST_RECEIVER` - (unique pour chaque ticket + store the LST)
PAYER = `WALLET` - (wallet du client)

**------Function create_collection:**

1- Le `SC` verifie que les settings de la collection sont valide (nom, prix, %, epoch, etc...). (`SETTING_SECURITY_FILTER`)

2- Le `SC` mint le NFT collection,

- Le `SC` set le `PDA A` comme first creator de la collection - verify signature
- Le `SC` set le `PDA B` comme second creator de la collection - verify signature

# Ainsi le NFT collection est minté que si les settings sont OK avec la politique MagicLottery.

# Le fait que PDA A ai signé la verification createur, confirme que le NFT a été minté par le SC, et que SETTING_SECURITY_FILTER a validé la creation.

**------Function buy_ticket:**

1-Le `SC` verifie que le montant de LAMPBPORT (SOL) demandé pour mint correspond bien au montant stipulé dans le `PDA B` de la collection.

2- Le `SC` mint le cNFT `Ticket` pour le client.

- Le `SC` set le `PDA B` comme first creator du cNFT `Ticket` - verify signature
- Le `SC` set le `PDA C` comme receiver des LST - verify signature

# Ainsi le cNFT `Ticket` est minté que si le montant de LAMPBPORT (SOL) correspond au montant stipulé dans le `PDA B` de la collection.

# Le fait que `PDA B` ai signé la verification createur, confirme que le cNFT a été minté par le `SC`, et que le montant de LAMPBPORT (SOL) est correct.

# Le fait que `PDA C` ai signé confirme que le `SC` a accepté de recevoir les LST sur `PDA C`.

# Le `PDA C` est unique pour chaque cNFT `Ticket` et indique ou sont stockés les LST de ce `Ticket`.

**------Function dissolve_ticket:**

1- Le `SC` verifie que le `PDA C` est bien un PDA du `SC` et que `PDA C` est bien un creator verifié sur le cNFT `Ticket`.
2- Le `SC` burn le cNFT `Ticket`.
3- Le `SC` withdraw les LST de `PDA C` et fait forward les SOL au wallet du `Payer`.

# Le fait que `PDA C` ai signé la verification createur, confirme que le `Ticket` a été minté par le `SC`, et que le `SC` a accepté de recevoir les LST sur `PDA C`.
