import time
#! /usr/bin/env python
# Author : Rahul Sharma

# import pyPdf

# opcodes = []

# def getPDFContent(path):
#     content = ""
#     pdf = pyPdf.PdfFileReader(file(path, "rb"))
#     for i in range(0, pdf.getNumPages()):
#     	content += pdf.getPage(i).extractText() + "\n"
#     with open('opcode.txt','w') as f:
#     	for i in range(2,248):
#     		r_ix = content.find(str(i)+'.')
# 			l_ix = content.find(str(i-1)+'.')
# 			f.write(content[l_ix:r_ix])
# 			f.write('\n')

# try:
# 	getPDFContent("opcode.pdf").encode("ascii","ignore")
# except:
# 	print("")
# 	print("File Error")

a=None
f = open('opcode.txt','r')
rs = open("opcode.rs","w")
a = f.readline()
while(a!=''):
	#a.split()
	if len(a.split()) <7:
		a = a.split()
		print("pub const "+a[1].lower()+"_"+a[2].lower()+":"+"(&\'static str,u8) = (\""+a[1].lower()+" "+a[2].lower()+"\""+","+a[3]+");"+"\n")
		rs.write("pub const "+a[1].lower()+"_"+a[2].lower()+":"+"(&\'static str,u8) = (\""+a[1].lower()+" "+a[2].lower()+"\""+","+a[3]+");"+"\n")
	a=f.readline()
f.close()
rs.close()