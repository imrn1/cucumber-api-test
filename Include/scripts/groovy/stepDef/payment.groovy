package stepDef
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

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

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class payment {

	@Given("Do payment with parameters (.*) (.*) (.*) (.*) (.*) (.*) (.*) (.*) (.*) (.*) (.*) (.*)")
	def Payment(String ext_transaction_id,String sender_account_number, String sender_wallet_number, String currency_code,
			String amount, String description,String receiver_wallet_number,String source_type, String channel_type,String hash_key,
			String earn,String burn) {
		CommonFunctions.body = """{
		 "ext_transaction_id": "${ext_transaction_id}",
		 "sender_account_number": "${sender_account_number}",
		 "sender_wallet_number": "${sender_wallet_number}",
		 "currency_code": "${currency_code}",
		 "amount": "${amount}",
		 "description": "${description}",
		 "receiver_wallet_number": "${receiver_wallet_number}",
		 "source_type": "${source_type}", 
		 "channel_type": "${channel_type}",
		 "hash_key": "${hash_key}",
		 "loyalty": {
		 "earn": "${earn}",
		 "burn": "${burn}"
		 }
		 }"""
		println("body : \n"+CommonFunctions.body)
	}
}