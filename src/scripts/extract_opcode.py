#! /usr/bin/env python
# Author : Rahul Sharma

import pyPdf
import time

def getPDFContent(path):
    content = ""
    pdf = pyPdf.PdfFileReader(file(path, "rb"))
    for i in range(0, pdf.getNumPages()):
    	content += pdf.getPage(i).extractText() + "\n"
    	
    for i in range(2,248):
		r_ix = content.find(str(i)+'.')
		l_ix = content.find(str(i-1)+'.')
		time.sleep(0.07)
		print content[l_ix:r_ix]
			
try:
	getPDFContent("opcode.pdf").encode("ascii","ignore")
except:
	print("")
	print("Dead End Man")