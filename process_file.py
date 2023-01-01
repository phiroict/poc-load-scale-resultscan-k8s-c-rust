ok = 0
nok = -1
counter = 0
with open("testresults.jtl", 'r') as reader:
    line = reader.readline()
    while line is not None:
        cells = line.split(",")
        # print("HTTP result {}".format(cells[3]))
        line = reader.readline()
        counter += 1
        if len(cells) > 4 and cells[3] == "200":
            ok += 1
        else:
            nok += 1
        if counter % 1000000 == 0:
            print("Records ok: {}, nok: {}".format(str(ok), str(nok)))
print("ENDRESULT: Records ok: {}, nok: {}".format(str(ok), str(nok)))