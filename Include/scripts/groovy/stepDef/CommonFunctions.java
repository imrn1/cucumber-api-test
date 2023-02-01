package stepDef;
import internal.GlobalVariable;
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint;
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase;
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData;
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject;
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject;

import com.kms.katalon.core.annotation.Keyword;
import com.kms.katalon.core.checkpoint.Checkpoint;
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords;
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords;
import com.kms.katalon.core.model.FailureHandling;
import com.kms.katalon.core.testcase.TestCase;
import com.kms.katalon.core.testdata.TestData;
import com.kms.katalon.core.testobject.ResponseObject;
import com.kms.katalon.core.testobject.TestObject;
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords;
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords;
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords;

import com.kms.katalon.core.testobject.ResponseObject;
import customKeyword.CustomMethods;
import customKeyword.Verify;


public class CommonFunctions {
	
	public static String body = null;
	public static String url = null;
	public static String method = "POST";
	public static ResponseObject response = null;


	public static String fileName = null;
	public static String process_description = "";
	public static String expected_status = "";
	public static String expected_code = "";
	public static String expected_message = "";
	public static String expected_ResponseStatusCode = "";
	public static Boolean isWrite = false;
	
	
	
	public static ResponseObject sendRequest() {
		
		response = CustomMethods.sendRequest(url, body, method, process_description,expected_status,expected_code,expected_message,
				expected_ResponseStatusCode, fileName, isWrite );
	
		return response;	
	}
	
	
	public static void verify(String key, String value) {
		Verify.VerifyElementPropetyValue(value.toString(), key ,response);
	}
	
	public static void clearDatas() {
		fileName = null;
		process_description = "";
		expected_status = "";
		expected_code = "";
		expected_message = "";
		expected_ResponseStatusCode = "";
		isWrite = false;
	}

}