package customKeyword

import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.RestRequestObjectBuilder
import com.kms.katalon.core.testobject.TestObjectProperty
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.testobject.ResponseObject

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

import java.time.format.DateTimeFormatter
import java.nio.file.WatchService
import java.time.LocalDateTime as LocalDateTime
import java.util.HashMap;
import customKeyword.Account as Account

import com.kms.katalon.core.util.KeywordUtil;
import org.json.JSONObject;
import writeFile.WriteFile as WF;



import com.kms.katalon.core.configuration.RunConfiguration
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import java.io.File




public class CustomMethods<T>{

	public static void setGlobalValueNull(String globalVariable){
		//println("globalVariable before : "+ GlobalVariable[String.valueOf(globalVariable)] )
		GlobalVariable[String.valueOf(globalVariable)] = null;
		//println("globalVariable after : "+ GlobalVariable[String.valueOf(globalVariable)] )
	}


	public static Boolean isNullOrEmpty(String str) {
		return (str == null || str.allWhitespace || str =="null")
	}


	public static String GenerateUniqeNumber(Integer characterNumber = 14 ) {

		//DateTimeFormatter dtf = DateTimeFormatter.ofPattern('SSmmyyyymmddHHSS')
		DateTimeFormatter dtf = DateTimeFormatter.ofPattern('nsmmyyyymmddHHSS')
		LocalDateTime now = LocalDateTime.now()

		characterNumber =  characterNumber > 14 ? 14 : characterNumber

		def ID = dtf.format(now)
		println("generated num : "+ ID)

		//println("generated uniqe_id : "+ ID)
		return ID.substring(0,characterNumber)
	}


	public static void wait(int ms) {
		println("wait : "+ms)
		try {
			Thread.sleep(ms);
		}
		catch(InterruptedException ex) {
			Thread.currentThread().interrupt();
		}
	}




	public static String FirstAccountNumberFromPersonalListByKycLevel(Integer level=0) {

		String body = """{
		  "page_size": 1,
		  "page_index": 1,
		  "order_column": "Id",
		  "order_by": "asc",
		  "kyc_level": $level
		}"""

		ResponseObject response = sendRequest("/v1/Account/PersonalListByFilter", body )

		try {
			return WS.getElementPropertyValue(response, "payload.results[0]['account_number']")
		}catch(Exception e) {
			return ""
		}
	}

	public static String FirstAccountNumberFromBusinessListByKycLevel(Integer level=0) {
		String body = """{
		  "page_size": 1,
		  "page_index": 1,
		  "order_column": "Id",
		  "order_by": "asc",
		  "kyc_level": $level
		}"""
		ResponseObject response = sendRequest("/v1/Account/BusinessListByFilter", body)

		try {
			return WS.getElementPropertyValue(response, "payload.results[0]['account_number']")
		}catch(Exception e) {
			return ""
		}
	}



	public static String VerifyKyc(Account account ,String level) {
		switch(level){
			case "10" :
			//WS.verifyEqual(account.kycInfo.kycLevel, Enums.KycLevel.Unknown)
				Verify.VerifyElementPropetyValue("10","payload.results[0].user_kyc_info.kyc_level" )
				return "kyc level doğrulaması başarılı"
			case "20" :
			//WS.verifyEqual(account.kycInfo.kycLevel, Enums.KycLevel.Unverified)
				Verify.VerifyElementPropetyValue("20","payload.results[0].user_kyc_info.kyc_level" )
				return "kyc level doğrulaması başarılı"
			case "30" :
			//WS.verifyEqual(account.kycInfo.kycLevel, Enums.KycLevel.Verified)
				Verify.VerifyElementPropetyValue("30","payload.results[0].user_kyc_info.kyc_level" )
				return "kyc level doğrulaması başarılı"
			case "40" :
			//WS.verifyEqual(account.kycInfo.kycLevel, Enums.KycLevel.Contracted)
				Verify.VerifyElementPropetyValue("40","payload.results[0].user_kyc_info.kyc_level" )
				return "kyc level doğrulaması başarılı"

			default :
			/*KeywordUtil.markWarning("Kyc tanımlanamadı (parametre ya da account nesnesi kyc bilgisi hatalı)")
			 KeywordUtil.logInfo("Kyc tanımlanamadı (parametre ya da account nesnesi kyc bilgisi hatalı)")
			 KeywordUtil.markFailedAndStop("Kyc tanımlanamadı (parametre ya da account nesnesi kyc bilgisi hatalı)")
			 */
				throw new Exception("Kyc tanımlanamadı (parametre ya da account nesnesi kyc bilgisi hatalı)")
		}
	}


























	/**
	 * token global değişkenden alınır. --> 401 hatası alınırsa istek tekrar gönderilir.
	 * default requestMethod = POST
	 * @param endpoint
	 * @param body
	 * @param requestMethod
	 * @param process_description
	 * @param expected_status
	 * @param expected_code
	 * @param expected_message
	 * @param expected_ResponseStatusCode
	 * @param fileName
	 * @return ResponseObject
	 */
	public static ResponseObject sendRequest(String endpoint, String body, String requestMethod="POST",
			String process_description="", String expected_status="", String expected_code="", String expected_message="",
			String expected_ResponseStatusCode="", String fileName="",Boolean isWrite=false ) {

		RequestObject ro = new RequestObject(GenerateUniqeNumber(8))
		ro.setRestUrl(GlobalVariable.sts_host+endpoint)

		ro = setRequestHeader(ro)

		ro.setRestRequestMethod(requestMethod)
		ro.setBodyContent(new HttpTextBodyContent(body))

		//JSONObject json = new JSONObject(ro.getBodyContent()["text"]);
		JSONObject json = new JSONObject(body);
		println("servis : "+endpoint +"\nrequest body: \n"+json.toString(4) );


		ResponseObject respObj = WS.sendRequest(ro)

		if(WS.getResponseStatusCode(respObj) == 401) {
			println("401 hatası alındı...\nToken alınıyor")

			WebUI.callTestCase(findTestCase('_Custom Cases/_Token alma'), [('token') : GlobalVariable.token], FailureHandling.STOP_ON_FAILURE)
			ro = setRequestHeader(ro)
			respObj = WS.sendRequest(ro)
		}

		WS.verifyNotEqual(WS.getResponseStatusCode(respObj), 401)
		json = new JSONObject(respObj.getBodyContent().getText());
		println("servis : "+endpoint +"\nresponse body\n"+json.toString(4));

		/************ dosyaya yazma ***********/

		if(isWrite) {

			String[] s = endpoint.split("/")
			//println("servis adı : => " + s[s.length-1])
			if(fileName == "")
				fileName = s[s.length-1]

			WF.writeExelFile(ro,respObj, s[s.length-1], process_description, expected_status, expected_code,
					expected_message,expected_ResponseStatusCode, fileName )
		}
		return respObj
	}




	/********************************************************************************************************/

	private static RequestObject setRequestHeader(RequestObject req ) {

		TestObjectProperty header1 = new TestObjectProperty("Authorization", ConditionType.EQUALS, "Bearer "+GlobalVariable.token)
		TestObjectProperty header2 = new TestObjectProperty("Content-Type", ConditionType.EQUALS, "application/json")
		TestObjectProperty header3 = new TestObjectProperty("Accept", ConditionType.EQUALS, "*/*")
		ArrayList defaultHeaders = Arrays.asList(header1, header2, header3)

		req.setHttpHeaderProperties(defaultHeaders)

		return req
	}



	/***
	 * servis çağrısı ise ilgili servisten dönen cevap dönülür değilse parametre olarak alınan deger dönülür.
	 * @param variable
	 * @return
	 */
	public static isServiceCall(String variable) {
		def value = null
		boolean flag = false

		if( (variable.startsWith("'") && variable.endsWith("'")  ) || (variable.startsWith('"') && variable.endsWith('"')  ) ) {
			flag = true
			variable = variable.substring(1, variable.size()-1)
		}


		if(variable.startsWith("@")) {
			//	println("bu bir servis çağrısıdır : " + variable + "\nyönlendirilmeli")
			value = callService(variable)
		}
		else {
			//	println("normal bir deger : "+variable)
			value = variable
		}

		//////////////////////////

		if(flag) {
			//	println("niçerikte ' işareti var ")
			return "'"+ value + "'"
		}
		//	println("içerikte ' işareti yok ")
		return value
	}



	private static callService(String serviceCode) {

		// @G-5,asd
		serviceCode = serviceCode.replace("@", "")
		println("serviceCode : @ den arındırılmış : "+serviceCode)

		String code = serviceCode.split("-")[0].toUpperCase()
		println("code : "+code)

		String[] params = []
		if(serviceCode.contains("-")) {
			params = serviceCode.split("-")[1].split(",")
		}
		println("params[] : "+params.toString() +" length : "+params.length)

		switch(code) {

			case "G":
				if(params.length > 0) {
					//println("params[0] : "+params[0])
					return GenerateUniqeNumber(params[0].toInteger())
				}
				return GenerateUniqeNumber()

			case "PFAN":
				if(params.length > 0) {   // @FAN-40
					return FirstAccountNumberFromPersonalListByKycLevel(params[0].toInteger())
				}
				return FirstAccountNumberFromPersonalListByKycLevel()

			case "BFAN":
				if(params.length > 0) {   // @FAN-40
					return FirstAccountNumberFromBusinessListByKycLevel(params[0].toInteger())
				}
				return FirstAccountNumberFromBusinessListByKycLevel()


			case "ACC":

				Account account = new Account();

				if( params[0].toLowerCase() == "?p" || params[0].toLowerCase() == "?b" ) { // @ACC-?p,b_1 -> params : [ '?p' , 'b_1']

					println("ACC ?? account type :"+params[0].toLowerCase())
					// tüm durumlarda verilen hesap no ile kayıt var mı sorgulanmalı
					String accountNumber = GlobalVariable.pre_account_number + params[1]
					account.getAccountByAccountNumber(accountNumber,
							params[0].toLowerCase() == "?b" ? Enums.AccountType.business : Enums.AccountType.personal) // kayıt bulunmazsa hata mesajı verir
					params = params.drop(1)

				}else {
					// tüm durumlarda verilen hesap no ile kayıt var mı sorgulanmalı
					String accountNumber = GlobalVariable.pre_account_number + params[0]
					account.getAccountByAccountNumber(accountNumber)  // kayıt bulunmazsa hata mesajı verir
				}


				if( params.length == 1 ) { //  @ACC-b_1 => account dönmeli
					//return GlobalVariable.pre_account_number + params[0]
					return account
				}
				else if(params.length >= 2) {
					if(params[1].toLowerCase() == "accno") {
						return account.AccountNumber
					}

					if(params[1].toLowerCase() == "w"){  // @ACC-b_1,w    => cüzdan listesindeki 0. elemana ait cüzdan no döner
						return account.Wallets[0].Number
					}
					if(params[1].toLowerCase() == "kyc"){  // @ACC-b_1,L => kyc level dönmeli : string olarak : unknown/verified
						return account.kycInfo.kycLevel
					}
					if(params[1].toLowerCase(new Locale("en")) == "nid"){  // @ACC-b_1,N => nationalID dönmeli
						return account.kycInfo.NationalID
					}

					if(params[1].toLowerCase() == "vkyc") {  // ACC-b_1,VKL,40
						return VerifyKyc( account ,params[2])
					}

					if(params[1].toLowerCase() == "defw") {  // hesaba ait default cüzdan no döner
						// buraya kadar getAccountByAccountNumber() fonk. çağırıldı
						// DefaultWalletNumber degeri doldurulmuştur
						return account.DefaultWalletNumber
					}
				}


		}

	}













	/*--------------------------------------------------------*/

	public static void getTCNOList() {

		String fileContents = new File(RunConfiguration.getProjectDir() + "/Data Files/exel/TC no list-json.json").getText("utf-8")

		def jsonSlurper = new JsonSlurper()
		def jsonObject = jsonSlurper.parseText(fileContents)

		println JsonOutput.prettyPrint(fileContents).toString()

		println("tostr: "+jsonObject.toString())
	}








}




