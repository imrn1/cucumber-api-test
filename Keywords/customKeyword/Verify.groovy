package customKeyword

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import com.kms.katalon.core.util.KeywordUtil;
import com.kms.katalon.core.testobject.ResponseObject

import customKeyword.CustomMethods


public class Verify {

	public static void VerifyElementPropetyValue(String expected_value,String responseProp,ResponseObject response = GlobalVariable.responseObject) {


		if(expected_value.toString().replaceAll(" ", "") != "") {

			//println("verify if içi : expected_value= "+expected_value)
			String responsePropValue = WS.getElementPropertyValue(response, responseProp).toString()
			boolean check = responsePropValue == expected_value ? true: false

			if(check) {
				println("expected_"+responseProp +" doğrulaması başarılı >> expected_"+responseProp  +" : "+expected_value +" response "+responseProp + " " +responsePropValue)
				KeywordUtil.logInfo( "expected_"+responseProp +" doğrulaması başarılı >> expected_"+responseProp  +" : "+expected_value +" response "+responseProp + " " +responsePropValue)
			}else {
				KeywordUtil.markWarning("expected_"+responseProp +" doğrulaması başarısız \nexpected_"+responseProp  +" : "+expected_value +"\nresponse "+responseProp + " : " +responsePropValue)
				KeywordUtil.logInfo("expected_"+responseProp +" doğrulaması başarısız \nexpected_"+responseProp  +" : "+expected_value +"\nresponse "+responseProp + " " +responsePropValue)
				KeywordUtil.markFailedAndStop("expected_"+responseProp +" doğrulaması başarısız")
			}
		}
		else {
			println("expected_value : '' (boş değer)")
		}
	}

	public static void VerifyResponseStatusCode(String expected_value, ResponseObject response = GlobalVariable.responseObject) {

		if(expected_value.toString().replaceAll(" ", "") != "") {
			String responsePropValue = WS.getResponseStatusCode(response).toString()
			boolean check = responsePropValue == expected_value ? true: false

			if(check) {
				//	KeywordUtil.logInfo( "expected_response_status_code doğrulaması başarılı >> expected_response_status_code : "+expected_value +" response_status_code" +responsePropValue)
				KeywordUtil.markPassed("expected_response_status_code doğrulaması başarılı >> expected_response_status_code : "+expected_value +" response_status_code" +responsePropValue)
			}else {
				KeywordUtil.markWarning("expected_response_status_code doğrulaması başarısız >> expected_response_status_code : "+expected_value +" response_status_code" +responsePropValue)
				KeywordUtil.logInfo("expected_response_status_code doğrulaması başarısız >> expected_response_status_code : "+expected_value +" response_status_code" +responsePropValue)
				KeywordUtil.markFailedAndStop("expected_response_status_code doğrulaması başarısız")
			}
		}
	}
}
