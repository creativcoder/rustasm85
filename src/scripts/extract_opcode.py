#! /usr/bin/env python
# Author : Rahul Sharma

import pyPdf

opcodes = []

def getPDFContent(path):
    content = ""
    pdf = pyPdf.PdfFileReader(file(path, "rb"))
    for i in range(0, pdf.getNumPages()):
    	content += pdf.getPage(i).extractText() + "\n"
    with open('opcode.txt','w') as f:	
    	for i in range(2,248):
			r_ix = content.find(str(i)+'.')
			l_ix = content.find(str(i-1)+'.')
			f.write(content[l_ix:r_ix])
			f.write('\n')
			
try:
	getPDFContent("opcode.pdf").encode("ascii","ignore")
except:
	print("")
	print("File Error")

a=None

f = open('opcode.txt','r')
while(a!=''):
	a=f.readline()
	#a.split()
	a = a.split()
	try:
		a[0] = '/*' + a[0] + '*/'
		a[len(a)-1] = '/*' + a[len(a)-1] + '*/'
		if len(a) <7:
			code_line = 'const ' + " ".join(a)
			print code_line
	except:
		a=f.readline()


