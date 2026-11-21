boy = input("Kahve boyu seçin (k/o/b): ")

match boy:
    case "k":
        ad, fiyat = "küçük", 70
    case "o":
        ad, fiyat = "orta", 85
    case "b":
        ad, fiyat = "büyük", 100
    case _:
        ad, fiyat = "tanımsız", 0

if fiyat == 0:
    print(f"'{boy}' diye bir boy yok.")
else:
    print(f"{ad.title()} kahve: {fiyat} TL")