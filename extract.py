f = open("itens.txt", "r")
for line in f:
    line = line.strip()
    # print(line)
    field, dastype = line.split(":")
    print("pub fn ",field,"(mut self, input: &'a str) -> Self {",sep="")
    print("\tself.query.push((\"",field,"\", input));",sep="")
    print("\tself")
    print("}")


    # pub fn name(mut self, input: &'a str) -> Self {
    #     self.query.push(("name", input));
    #     self
    # }