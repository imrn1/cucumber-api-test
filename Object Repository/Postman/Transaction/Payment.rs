<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Payment</name>
   <tag></tag>
   <elementGuidId>6a31fd01-765d-4a0a-b36f-3379cc72d0f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n \&quot;ext_transaction_id\&quot;: \&quot;${ext_transaction_id}\&quot;,\n \&quot;sender_account_number\&quot;: \&quot;${sender_account_number}\&quot;,\n  \&quot;sender_wallet_number\&quot;: \&quot;${sender_wallet_number}\&quot;,\n  \&quot;currency_code\&quot;: \&quot;${currency_code}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;description\&quot;: \&quot;${description}\&quot;,\n  \&quot;receiver_wallet_number\&quot;: \&quot;${receiver_wallet_number}\&quot;,\n\n   \&quot;source_type\&quot;: \&quot;${source_type}\&quot;, \n   \&quot;channel_type\&quot;: \&quot;${channel_type}\&quot;,\n  \&quot;hash_key\&quot;: \&quot;${hash_key}\&quot;,\n \n    \&quot;loyalty\&quot;: {\n    \&quot;earn\&quot;: \&quot;${earn}\&quot;,\n    \&quot;burn\&quot;: \&quot;${burn}\&quot;\n  }\n}\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f80bdd5a-2aeb-4f60-bdd5-ca69289a9274</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>9f911c56-acfd-4bd8-8762-bf9e6ac58bcc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sts_host}/v1/Transaction/Payment</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sts_host</defaultValue>
      <description></description>
      <id>5c928dba-0a4e-49c2-b73c-feee526c2c52</id>
      <masked>false</masked>
      <name>sts_host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ca93b151-ee35-4184-aa79-a019eba0fea1</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a4f42372-b5bd-4b23-beee-a9af9a7b011b</id>
      <masked>false</masked>
      <name>ext_transaction_id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>179e43ca-dcf4-43ea-b3d7-e9f9de3e662e</id>
      <masked>false</masked>
      <name>sender_account_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fa354f2c-480a-4364-a5e9-e36e76db84ac</id>
      <masked>false</masked>
      <name>sender_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b3910e89-8493-4c2c-a37e-97cf0a877558</id>
      <masked>false</masked>
      <name>currency_code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ffa143b3-fe3a-4349-828f-28d02e761320</id>
      <masked>false</masked>
      <name>description</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bfda8e86-7fc1-4fce-9fe7-43c44427abe4</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cdc6d2cc-7189-4285-b17f-69ce72780b34</id>
      <masked>false</masked>
      <name>receiver_wallet_number</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>2d48ffe2-1292-420d-bcf0-0a8807320bcb</id>
      <masked>false</masked>
      <name>earn</name>
   </variables>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>c079ee04-d307-4291-8ed2-9b264d1e115b</id>
      <masked>false</masked>
      <name>burn</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>34b17c47-2fcc-4566-939d-8d8f0fad11d8</id>
      <masked>false</masked>
      <name>source_type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f461e8c4-b497-463e-9053-3c18b96f60b1</id>
      <masked>false</masked>
      <name>hash_key</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>00e1a8a3-02d3-4394-a770-0b7d357204cb</id>
      <masked>false</masked>
      <name>channel_type</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import org.json.JSONObject;
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


GlobalVariable.responseObject = response
GlobalVariable.requestObject = request


println(&quot;Payment request body: \n&quot;+ request.getBodyContent().getText())

WS.verifyNotEqual(WS.getResponseStatusCode(response), 401)


import org.json.JSONObject;
JSONObject json = new JSONObject(response.getBodyContent().getText()); // Convert text to object
println(&quot;Payment response body\n&quot;+json.toString(4));



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
