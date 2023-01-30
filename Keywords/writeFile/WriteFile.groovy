package writeFile
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import java.time.format.DateTimeFormatter as DateTimeFormatter
import java.time.LocalDateTime as LocalDateTime


import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;

import org.apache.poi.EncryptedDocumentException;
import org.apache.poi.openxml4j.exceptions.InvalidFormatException;
import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.ss.usermodel.WorkbookFactory;

import org.apache.poi.hssf.usermodel.HSSFWorkbook;
import org.apache.poi.xssf.usermodel.XSSFSheet;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;

import org.json.JSONObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS


class WriteFile {

	DateTimeFormatter dtf = DateTimeFormatter.ofPattern('yyyy-MM-dd-HH')
	LocalDateTime now = LocalDateTime.now()

	String excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now) + '.xlsx'

	static boolean isNullOrEmpty(String str) {
		return (str == null || str.allWhitespace)
	}


	@Keyword
	def writeExel(RequestObject request = GlobalVariable.requestObject ,ResponseObject response=GlobalVariable.responseObject,
			String serviceName="" ,String processDescription="", String expected_status="" ,String expected_code="",
			String expected_message="", String fileName ="",String expected_response_status_code="" ) {

		if(!isNullOrEmpty(fileName))
			excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now)+"_" + fileName +'.xlsx'

		File file = new File(excelFilePath);

		boolean isNewFile = !file.exists()

		if(isNewFile) {
			println("keyword içinden : new file :"+isNewFile)

			XSSFWorkbook workbook1 = new XSSFWorkbook();
			XSSFSheet sheet = workbook1.createSheet("Sheet1");

			int rowCount = 0;
			int columnCount = 0;

			Row row = sheet.createRow(rowCount);
			Cell cell = row.createCell(columnCount);
			cell.setCellValue("process description");

			cell = row.createCell(++columnCount);
			cell.setCellValue("service name");

			cell = row.createCell(++columnCount);
			cell.setCellValue("request");

			cell = row.createCell(++columnCount);
			cell.setCellValue("response");

			cell = row.createCell(++columnCount);
			cell.setCellValue("response status code");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected response status code");

			cell = row.createCell(++columnCount);
			cell.setCellValue("status");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_status");

			cell = row.createCell(++columnCount);
			cell.setCellValue("code");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_code");

			cell = row.createCell(++columnCount);
			cell.setCellValue("message");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_message");


			FileOutputStream outputStream = new FileOutputStream(excelFilePath)
			workbook1.write(outputStream);
			workbook1.close();
			outputStream.close();
		}

		FileInputStream inputStream = new FileInputStream(new File(excelFilePath));
		Workbook workbook = WorkbookFactory.create(inputStream);

		Sheet sheet = workbook.getSheetAt(0);
		int rowCount = sheet.getLastRowNum();

		Row row = sheet.createRow(++rowCount);

		int columnCount = 0;

		Cell cell = row.createCell(columnCount);
		cell.setCellValue(processDescription); // description

		cell = row.createCell(++columnCount);
		cell.setCellValue(serviceName); // serviceName


		cell = row.createCell(++columnCount);  // request
		try {
			if((RequestObject)GlobalVariable.requestObject != null) {
				def json = new JSONObject( ((RequestObject)GlobalVariable.requestObject).getBodyContent()["text"]);
				cell.setCellValue(json.toString(4));
			}
		}catch(Exception ex){
			cell.setCellValue(((RequestObject)GlobalVariable.requestObject).getBodyContent().getText());
		}


		cell = row.createCell(++columnCount);  // response
		try {
			if((ResponseObject)GlobalVariable.responseObject != null) {
				def json = new JSONObject( ((ResponseObject)GlobalVariable.responseObject).getBodyContent().getText());
				cell.setCellValue(json.toString(4).substring(0,200));
				// 200 karakter
			}
		}catch(Exception ex){
			cell.setCellValue(((ResponseObject)GlobalVariable.responseObject).getBodyContent().getText());
		}




		cell = row.createCell(++columnCount);
		cell.setCellValue(WS.getResponseStatusCode(((ResponseObject)GlobalVariable.responseObject)));   // response status code
		cell = row.createCell(++columnCount);  // expected_response_status_code
		cell.setCellValue(expected_response_status_code);


		cell = row.createCell(++columnCount);  // status
		try {
			cell.setCellValue(WS.getElementPropertyValue(GlobalVariable.responseObject, "status"));  //  status
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);  // expected_status
		cell.setCellValue(expected_status);

		cell = row.createCell(++columnCount);
		try {
			cell.setCellValue(WS.getElementPropertyValue(GlobalVariable.responseObject, "code"));  //  code
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);
		cell.setCellValue(expected_code);  // expected_code

		cell = row.createCell(++columnCount);
		try {
			cell.setCellValue(WS.getElementPropertyValue(GlobalVariable.responseObject, "message"));  //  message
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);
		cell.setCellValue(expected_message);   // expected_message

		row = sheet.createRow(++rowCount);


		inputStream.close();

		FileOutputStream outputStream = new FileOutputStream(excelFilePath);
		workbook.write(outputStream);
		workbook.close();
		outputStream.close();

	}



	@Keyword
	def boolReturnedServices(String serviceName="" ,  String fileName ="" ) {

		def fileLoc = "/STS API test/Data Files/logs/bool dönen servisler"

		if(!isNullOrEmpty(fileName))
			excelFilePath = GlobalVariable.katalon_std_location + fileLoc + fileName +'.xlsx'

		File file = new File(excelFilePath);

		boolean isNewFile = !file.exists()

		if(isNewFile) {

			XSSFWorkbook workbook1 = new XSSFWorkbook();
			XSSFSheet sheet = workbook1.createSheet("Sheet1");

			int rowCount = 0;
			int columnCount = 0;

			Row row = sheet.createRow(rowCount);
			Cell cell = row.createCell(columnCount);
			cell.setCellValue("service name");


			FileOutputStream outputStream = new FileOutputStream(excelFilePath)
			workbook1.write(outputStream);
			workbook1.close();
			outputStream.close();

		}


		FileInputStream inputStream = new FileInputStream(new File(excelFilePath));
		Workbook workbook = WorkbookFactory.create(inputStream);

		Sheet sheet = workbook.getSheetAt(0);
		int rowCount = sheet.getLastRowNum();

		Row row = sheet.createRow(++rowCount);

		int columnCount = 0;

		///////////////////////////////////

		boolean isServiceNameInFile = false;
		Iterator<Row> rowIterator = sheet.iterator();
		while (rowIterator.hasNext()) {

			row = rowIterator.next();
			Iterator<Cell> cellIterator = row.cellIterator();

			while (cellIterator.hasNext()) {

				Cell cell = cellIterator.next();
				if(serviceName.toLowerCase() == cell.getStringCellValue().toLowerCase() ) {
					isServiceNameInFile = true;
					break;
				}

			}
		}

		if(!isServiceNameInFile) {
			row = sheet.createRow(++rowCount);
			Cell cell = row.createCell(columnCount);
			cell.setCellValue(serviceName);
		}


		inputStream.close();

		FileOutputStream outputStream = new FileOutputStream(excelFilePath);
		workbook.write(outputStream);
		workbook.close();
		outputStream.close();

	}



	@Keyword
	def WalletInfo(String description, String serviceName="" ) { //, String fileName =""

		if(!isNullOrEmpty(fileName))
			excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now)+"_WalletInfo" + '.xlsx'
		//excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now)+"_" + fileName +'.xlsx'

		File file = new File(excelFilePath);

		boolean isNewFile = !file.exists()

		if(isNewFile) {  // başlıklar oluşturulur
			println("keyword içinden : new file :"+isNewFile)

			XSSFWorkbook workbook1 = new XSSFWorkbook();
			XSSFSheet sheet = workbook1.createSheet("Sheet1");

			int rowCount = 0;
			int columnCount = 0;

			Row row = sheet.createRow(rowCount);

			Cell cell = row.createCell(columnCount);
			cell.setCellValue("Description");         // işlem öncesi-sonrası ..... receiver-sender /n (işlem no)

			cell = row.createCell(++columnCount);
			cell.setCellValue("Account Info");     // account_number : .....  /n wallet_number : .......

			try {
				if((RequestObject)GlobalVariable.beforeTransactionSenderWalletInfo != null) {
					json = new JSONObject( ((RequestObject)GlobalVariable.beforeTransactionSenderWalletInfo).getBodyContent().getText());
					cell.setCellValue(json.toString(4));

				}
			}catch(Exception ex){
				cell.setCellValue(((RequestObject)GlobalVariable.requestObject).getBodyContent().getText());
			}






			cell = row.createCell(++columnCount);
			cell.setCellValue("Kyc Level");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Service Name");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Before Total Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("After Total Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Before Payment Available Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("After Payment Available Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Before Payment Unavailable Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("After Payment Unavailable Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Before Cash Available Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("After Cash Available Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("Before Cash Unavailable Balance");

			cell = row.createCell(++columnCount);
			cell.setCellValue("After Cash Unavailable Balance");


			FileOutputStream outputStream = new FileOutputStream(excelFilePath)
			workbook1.write(outputStream);
			workbook1.close();
			outputStream.close();
		}


		FileInputStream inputStream = new FileInputStream(new File(excelFilePath));
		Workbook workbook = WorkbookFactory.create(inputStream);

		Sheet sheet = workbook.getSheetAt(0);
		int rowCount = sheet.getLastRowNum();

		Row row = sheet.createRow(++rowCount);

		int columnCount = 0;

		Cell cell = row.createCell(columnCount);
		cell.setCellValue(description); // description

		cell = row.createCell(++columnCount);
		cell.setCellValue( ); // Account Info  => account_number : ...  /n wallet_number : ...

		cell = row.createCell(++columnCount);
		cell.setCellValue(KycLevel);

		cell = row.createCell(++columnCount);
		cell.setCellValue(serviceName);

		cell = row.createCell(++columnCount);
		cell.setCellValue(totalBalance); // Total Balance

		cell = row.createCell(++columnCount);
		cell.setCellValue(PaymentAvailableBalance); // Payment Available Balance

		cell = row.createCell(++columnCount);
		cell.setCellValue(PaymentUnvailableBalance); // Payment Unavailable Balance

		cell = row.createCell(++columnCount);
		cell.setCellValue(CashAvailableBalance); // Cash Available Balance

		cell = row.createCell(++columnCount);
		cell.setCellValue(CashUnvailableBalance); // Cash Unavailable Balance





		cell = row.createCell(++columnCount);  // request
		try {
			if((RequestObject)GlobalVariable.requestObject != null) {
				json = new JSONObject( ((RequestObject)GlobalVariable.requestObject).getBodyContent().getText());
				cell.setCellValue(json.toString(4));
			}
		}catch(Exception ex){
			cell.setCellValue(((RequestObject)GlobalVariable.requestObject).getBodyContent().getText());
		}



	}





	public static writeExelFile(RequestObject request = GlobalVariable.requestObject ,ResponseObject response=GlobalVariable.responseObject,
			String serviceName="" ,String processDescription="", String expected_status="" ,String expected_code="",String expected_message="",
			String expected_response_status_code="" , String fileName ="") {

		DateTimeFormatter dtf = DateTimeFormatter.ofPattern('yyyy-MM-dd-HH')
		LocalDateTime now = LocalDateTime.now()
		String excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now) + '.xlsx'


		if(!isNullOrEmpty(fileName))
			excelFilePath = GlobalVariable.katalon_std_location + GlobalVariable.exelFileLoc +"/"+ dtf.format(now)+"_" + fileName +'.xlsx'

		File file = new File(excelFilePath);

		boolean isNewFile = !file.exists()

		if(isNewFile) {
			//	println("new file :"+isNewFile)

			XSSFWorkbook workbook1 = new XSSFWorkbook();
			XSSFSheet sheet = workbook1.createSheet("Sheet1");

			int rowCount = 0;
			int columnCount = 0;

			Row row = sheet.createRow(rowCount);
			Cell cell = row.createCell(columnCount);
			cell.setCellValue("process description");

			cell = row.createCell(++columnCount);
			cell.setCellValue("service name");

			cell = row.createCell(++columnCount);
			cell.setCellValue("request");

			cell = row.createCell(++columnCount);
			cell.setCellValue("response");

			cell = row.createCell(++columnCount);
			cell.setCellValue("response status code");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected response status code");

			cell = row.createCell(++columnCount);
			cell.setCellValue("status");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_status");

			cell = row.createCell(++columnCount);
			cell.setCellValue("code");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_code");

			cell = row.createCell(++columnCount);
			cell.setCellValue("message");
			cell = row.createCell(++columnCount);
			cell.setCellValue("expected_message");


			FileOutputStream outputStream = new FileOutputStream(excelFilePath)
			workbook1.write(outputStream);
			workbook1.close();
			outputStream.close();
		}

		FileInputStream inputStream = new FileInputStream(new File(excelFilePath));
		Workbook workbook = WorkbookFactory.create(inputStream);

		Sheet sheet = workbook.getSheetAt(0);
		int rowCount = sheet.getLastRowNum();

		Row row = sheet.createRow(++rowCount);

		int columnCount = 0;

		Cell cell = row.createCell(columnCount);
		cell.setCellValue(processDescription); // description

		cell = row.createCell(++columnCount);
		cell.setCellValue(serviceName); // serviceName


		cell = row.createCell(++columnCount);  // request
		try {
			if((RequestObject)request != null) {
				//def json = new JSONObject( ((RequestObject)request).getBodyContent()["text"]);
				//println("----->>> req obj yazdırma <<<------")
				//def json = new JSONObject( ((RequestObject)request).getBodyContent().getText().toString());
				//cell.setCellValue(json.toString(4));

				cell.setCellValue(((RequestObject)request).getBodyContent().getText());
			}
		}catch(Exception ex){
			//println("----->>>catch : req obj yazdırma <<<------")
			cell.setCellValue(((RequestObject)request).getBodyContent().getText());
		}


		cell = row.createCell(++columnCount);  // response
		try {
			if((ResponseObject)response != null) {
				def json = new JSONObject( ((ResponseObject)response).getBodyContent().getText());
				cell.setCellValue(json.toString(4));
				//cell.setCellValue(json.toString(4).substring(0,500));
				// 500 karakter
			}
		}catch(Exception ex){
			cell.setCellValue(((ResponseObject)response).getBodyContent().getText());
		}




		cell = row.createCell(++columnCount);
		cell.setCellValue(WS.getResponseStatusCode(((ResponseObject)response)));   // response status code
		cell = row.createCell(++columnCount);  // expected_response_status_code
		cell.setCellValue(expected_response_status_code);


		cell = row.createCell(++columnCount);  // status
		try {
			cell.setCellValue(WS.getElementPropertyValue(response, "status"));  //  status
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);  // expected_status
		cell.setCellValue(expected_status);

		cell = row.createCell(++columnCount);
		try {
			cell.setCellValue(WS.getElementPropertyValue(response, "code"));  //  code
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);
		cell.setCellValue(expected_code);  // expected_code

		cell = row.createCell(++columnCount);
		try {
			cell.setCellValue(WS.getElementPropertyValue(response, "message"));  //  message
		}catch(Exception ex){}
		cell = row.createCell(++columnCount);
		cell.setCellValue(expected_message);   // expected_message

		row = sheet.createRow(++rowCount);


		inputStream.close();

		FileOutputStream outputStream = new FileOutputStream(excelFilePath);
		workbook.write(outputStream);
		workbook.close();
		outputStream.close();

	}










}






